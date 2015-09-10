use std::cell::RefCell;
use std::collections::HashMap;

use driver::cmd::Args;
use error::diag::Diagnostic;

use parser::ast::*;
use parser::ast::map::Map;
use parser::interner::*;

use sym::*;
use sym::Sym::*;

pub struct Context<'a, 'ast> where 'ast: 'a {
    pub args: &'a Args,
    pub interner: &'a Interner,
    pub map: &'a Map<'ast>,
    pub ast: &'a Ast,
    pub diag: RefCell<Diagnostic>,
    pub sym: RefCell<SymTable>,

    // points to the definition of variable from its usage
    pub defs: RefCell<HashMap<NodeId, VarInfoId>>,

    // maps function call to FctInfoId
    pub calls: RefCell<HashMap<NodeId, FctInfoId>>,

    // stores all function definitions
    pub fct_infos: RefCell<Vec<FctInfo<'ast>>>,

    // stores all var definitions
    pub var_infos: RefCell<Vec<VarInfo>>,
}

impl<'a, 'ast> Context<'a, 'ast> {
    pub fn new(args: &'a Args, interner: &'a Interner,
           map: &'a Map<'ast>, ast: &'a Ast) -> Context<'a, 'ast> {
        Context {
            args: args,
            interner: interner,
            map: map,
            ast: ast,
            diag: RefCell::new(Diagnostic::new()),
            sym: RefCell::new(SymTable::new()),
            defs: RefCell::new(HashMap::new()),
            calls: RefCell::new(HashMap::new()),
            fct_infos: RefCell::new(Vec::new()),
            var_infos: RefCell::new(Vec::new()),
        }
    }

    pub fn add_function(&self, fct_info: FctInfo<'ast>) -> Result<FctInfoId, Sym> {
        let name = fct_info.name;
        let fctid = FctInfoId(self.fct_infos.borrow().len());

        if let Some(ast) = fct_info.ast {
            assert!(self.calls.borrow_mut().insert(ast.id, fctid).is_none());
        }

        self.fct_infos.borrow_mut().push(fct_info);

        let mut sym = self.sym.borrow_mut();

        match sym.get(name) {
            Some(sym) => Err(sym),
            None => {
                assert!(sym.insert(name, SymFunction(fctid)).is_none());

                Ok(fctid)
            }
        }
    }

    pub fn add_var<F>(&self, var_info: VarInfo, replacable: F) ->
            Result<VarInfoId, Sym> where F: FnOnce(&Sym) -> bool {
        let name = var_info.name;
        let varid = VarInfoId(self.var_infos.borrow().len());

        let result = match self.sym.borrow().get(name) {
            Some(sym) => if replacable(&sym) { Ok(varid) } else { Err(sym) },
            None => Ok(varid)
        };

        if result.is_ok() {
            self.sym.borrow_mut().insert(name, SymVar(varid));
            assert!(self.defs.borrow_mut().insert(var_info.node_id, varid).is_none());
            self.var_infos.borrow_mut().push(var_info);
        }

        result
    }

    pub fn function<F, R>(&self, id: NodeId, f: F) -> R where F: FnOnce(&mut FctInfo<'ast>) -> R {
        let map = self.calls.borrow();
        let fctid = *map.get(&id).unwrap();

        let mut fct_infos = self.fct_infos.borrow_mut();
        f(&mut fct_infos[fctid.0])
    }

    pub fn var<F, R>(&self, id: NodeId, f: F) -> R where F: FnOnce(&mut VarInfo) -> R {
        let defs = self.defs.borrow();
        let varid = *defs.get(&id).unwrap();

        let mut var_infos = self.var_infos.borrow_mut();
        f(&mut var_infos[varid.0])
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FctInfoId(pub usize);

#[derive(Debug)]
pub struct FctInfo<'ast> {
    pub name: Name,
    pub params_types: Vec<BuiltinType>,
    pub return_type: BuiltinType,
    pub ast: Option<&'ast Function>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct VarInfoId(pub usize);

#[derive(Debug)]
pub struct VarInfo {
    pub name: Name,
    pub data_type: BuiltinType,
    pub node_id: NodeId,
}
