use parking_lot::RwLock;

use crate::error::msg::SemError;
use crate::semck::{report_term_shadow, report_type_shadow};
use crate::sym::{SymTable, TermSym, TypeSym};
use crate::vm::{EnumId, ImportData, NamespaceId, VM};

use dora_parser::interner::Name;

pub fn check<'a>(vm: &VM) {
    for import in &vm.imports {
        check_import(vm, import);
    }
}

fn check_import(vm: &VM, import: &ImportData) {
    let table = vm.namespace_table(import.namespace_id);

    let container_name = import.ast.container_name;
    let element_name = import.ast.element_name;
    let target_name = import.ast.target_name.unwrap_or(element_name);

    let sym_term = table.read().get_term(container_name);
    let sym_type = table.read().get_type(container_name);

    match (sym_term, sym_type) {
        (Some(TermSym::Namespace(namespace_id)), _) => {
            import_namespace(vm, import, &table, namespace_id, element_name, target_name)
        }

        (_, Some(TypeSym::Enum(enum_id))) => {
            import_enum(vm, import, &table, enum_id, element_name, target_name)
        }

        _ => {
            vm.diag.lock().report(
                import.file_id.into(),
                import.ast.pos,
                SemError::EnumExpected,
            );
        }
    }
}

fn import_namespace(
    vm: &VM,
    import: &ImportData,
    table: &RwLock<SymTable>,
    namespace_id: NamespaceId,
    element_name: Name,
    target_name: Name,
) {
    let namespace = &vm.namespaces[namespace_id.to_usize()];
    let sym_term = namespace.table.read().get_term(element_name);
    let sym_type = namespace.table.read().get_type(element_name);

    match (sym_term, sym_type) {
        (Some(TermSym::Fct(fct_id)), _) => {
            let new_sym = TermSym::Fct(fct_id);
            if let Some(old_sym) = table.write().insert_term(target_name, new_sym) {
                report_term_shadow(vm, target_name, import.file_id, import.ast.pos, old_sym);
            }
        }

        (_, Some(TypeSym::Class(cls_id))) => {
            let new_sym = TypeSym::Class(cls_id);
            if let Some(old_sym) = table.write().insert_type(target_name, new_sym) {
                report_type_shadow(vm, target_name, import.file_id, import.ast.pos, old_sym);
            }
        }

        (_, _) => unimplemented!(),
    }
}

fn import_enum(
    vm: &VM,
    import: &ImportData,
    table: &RwLock<SymTable>,
    enum_id: EnumId,
    element_name: Name,
    target_name: Name,
) {
    let xenum = vm.enums[enum_id].read();

    if let Some(&variant_id) = xenum.name_to_value.get(&element_name) {
        let sym = TermSym::EnumValue(enum_id, variant_id as usize);
        if let Some(sym) = table.write().insert_term(target_name, sym) {
            report_term_shadow(vm, target_name, import.file_id.into(), import.ast.pos, sym);
        }
    } else {
        let name = vm.interner.str(element_name).to_string();
        vm.diag.lock().report(
            import.file_id.into(),
            import.ast.pos,
            SemError::UnknownEnumValue(name),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::error::msg::SemError;
    use crate::semck::tests::*;

    #[test]
    fn check_initializer() {
        ok("let a: Int32 = 0;");
        ok("let a: Int32 = 0; var b: Int32 = a + 1;");
        err(
            "var a: Int32 = foo;",
            pos(1, 16),
            SemError::UnknownIdentifier("foo".into()),
        );
    }

    #[test]
    fn check_type() {
        err(
            "var x: Foo = 0;",
            pos(1, 8),
            SemError::UnknownIdentifier("Foo".into()),
        );
    }
}