use ast;
use ast::Ast;
use ast::Function;
use ast::Elem::ElemFunction;

use error::ErrorCode;
use error::ParseError;

use parser::interner::Interner;

use parser::Parser;

use sym;
use sym::BuiltinType;
use sym::SymTable;
use sym::Sym::{self, SymFunction};
use sym::SymFunctionType;
use sym::Type;

mod reachable;

macro_rules! err {
    ( $errs: expr, $x: expr ) => {
        {
            match $x {
                Ok(val) => val,
                Err(_) => { return Err($errs); }
            }
        }
    };
}

pub fn check<'a>(ast: &'a Ast, interner: &'a mut Interner) -> Result<SymTable, Vec<ParseError>> {
    SemCheck::new(ast, interner).check()
}

// Only do semantic analysis until some amount of
// errors found
static MAX_ERRORS: usize = 5;

struct SemCheck<'a> {
    ast: &'a Ast,
    interner: &'a mut Interner,
    errors: Vec<ParseError>,
}

impl<'a> SemCheck<'a> {
    fn new<'b>(ast: &'b Ast, interner: &'b mut Interner) -> SemCheck<'b> {
        SemCheck {
            ast: ast,
            interner: interner,
            errors: Vec::new(),
        }
    }

    fn check(mut self) -> Result<SymTable, Vec<ParseError>> {
        let mut globals = SymTable::new();

        add_predefined_types(self.interner, &mut globals);
        add_predefined_functions(self.interner, &mut globals);

        err!(self.errors, self.parse_function_headers(&mut globals));
        err!(self.errors, self.parse_function_bodies(&mut globals));

        if self.errors.is_empty() {
            Ok(globals)
        } else {
            Err(self.errors)
        }
    }

    fn parse_function_headers(&mut self, globals: &mut SymTable) -> Result<(), ()> {
        for elem in &self.ast.elements {
            match *elem {
                ElemFunction(ref fct) => {
                    try!(self.add_function_header(globals, fct));
                }

                _ => unreachable!()
            }
        }

        Ok(())
    }

    fn parse_function_bodies(&mut self, globals: &mut SymTable) -> Result<(), ()> {
        for fct in globals.functions_mut() {
            try!(self.check_function_body(fct));
        }

        Ok(())
    }

    fn parse_type(&mut self, globals: &SymTable, ty: &ast::Type) -> Result<sym::Type, ()> {
        let newtype = match *ty {
            ast::Type::TypeBasic(ref val) => {
                if let None = globals.find(val.name).map(|s| s.to_type()) {
                    let message = format!("unknown type {}", self.interner.str(val.name));

                    try!(self.error(ParseError {
                        code: ErrorCode::UnknownType,
                        message: message,
                        position: ty.pos()
                    }));

                    return Err(());
                }

                sym::Type::create_basic(val.name)
            }

            ast::Type::TypeTuple(ref val) => {
                let mut subtypes = Vec::with_capacity(val.subtypes.len());

                for ty in &val.subtypes {
                    let ty = box try!(self.parse_type(globals, ty));
                    subtypes.push(ty);
                }

                sym::Type::create_tuple(subtypes)
            }

            ast::Type::TypePtr(ref val) => {
                let subtype = box try!(self.parse_type(globals, &val.subtype));

                sym::Type::create_ptr(subtype)
            }

            ast::Type::TypeArray(ref val) => {
                let subtype = box try!(self.parse_type(globals, &val.subtype));

                sym::Type::create_array(subtype)
            }
        };

        Ok(newtype)
    }

    fn add_function_header(&mut self, globals: &mut SymTable, fct: &Function) -> Result<(), ()> {
        let params = try!(self.parse_function_params(globals, fct));

        let ty = if let Some(ref ty) = fct.return_type {
            try!(self.parse_type(globals, ty))
        } else {
            Type::create_unit()
        };

        let symfct = SymFunction(SymFunctionType {
            name: fct.name,
            return_type: ty,
            params: params,
            body: fct.block.id()
        });

        if let Err(_) = globals.insert(fct.name, symfct) {
            let message = format!("identifier {} already exists", self.interner.str(fct.name));

            try!(self.error(ParseError {
                code: ErrorCode::IdentifierAlreadyExists,
                message: message,
                position: fct.pos
            }));
        }

        Ok(())
    }

    fn parse_function_params(&mut self, globals: &mut SymTable, fct: &Function)
            -> Result<Vec<sym::Param>, ()> {
        let mut params = Vec::with_capacity(fct.params.len());

        for param in &fct.params {
            let p = try!(self.parse_param(globals, param));
            params.push(p);
        }

        Ok(params)
    }

    fn parse_param(&mut self, globals: &SymTable, param: &ast::Param)
            -> Result<sym::Param, ()> {
        let ty = try!(self.parse_type(globals, &param.data_type));

        Ok(sym::Param {
            name: param.name,
            data_type: ty
        })
    }

    fn check_function_body(&mut self, fct: &SymFunctionType) -> Result<(), ()> {
        Ok(())
    }

    fn error(&mut self, error: ParseError) -> Result<(), ()> {
        self.errors.push(error);

        if self.errors.len() <= MAX_ERRORS {
            Ok(())
        } else {
            Err(())
        }
    }

    fn check_function(&mut self, fct: &Function) {
        println!("check_function");
    }
}

fn add_predefined_types(interner: &mut Interner, globals: &mut SymTable) {
    let name = interner.intern(String::from("int"));
    let sym = Sym::create_type(name, BuiltinType::Int);
    globals.insert(name, sym);

    let name = interner.intern(String::from("str"));
    let sym = Sym::create_type(name, BuiltinType::Str);
    globals.insert(name, sym);

    let name = interner.intern(String::from("bool"));
    let sym = Sym::create_type(name, BuiltinType::Bool);
    globals.insert(name, sym);
}

fn add_predefined_functions(interner: &mut Interner, globals: &mut SymTable) {
    // TODO: add print(str), print_int(int)
}


#[test]
fn test_empty_file() {
    let (prog, mut interner) = Parser::from_str("").parse().unwrap();

    SemCheck::new(&prog, &mut interner).check().unwrap();
}

#[test]
fn test_function_multiple_times() {
    let (prog, mut interner) = Parser::from_str("fn main() {} fn main() {}").parse().unwrap();
    let errors = SemCheck::new(&prog, &mut interner).check().unwrap_err();

    assert_eq!(1, errors.len());
    assert_eq!(ErrorCode::IdentifierAlreadyExists, errors[0].code);
}

#[test]
fn test_builtin_types() {
    let (prog, mut interner) = Parser::from_str(
        "fn main(a: int, b: str, c: bool) { }").parse().unwrap();
    SemCheck::new(&prog, &mut interner).check().unwrap();
}

#[test]
fn test_unknown_type() {
    let (prog, mut interner) = Parser::from_str("fn main(a: abc) { }").parse().unwrap();
    let errors = SemCheck::new(&prog, &mut interner).check().unwrap_err();

    assert_eq!(1, errors.len());
    assert_eq!(ErrorCode::UnknownType, errors[0].code);
}

