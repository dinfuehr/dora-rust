use parking_lot::RwLock;
use std::sync::Arc;

use dora_parser::ast;
use dora_parser::interner::Name;
use dora_parser::lexer::position::Position;

use crate::ty::SourceType;
use crate::utils::GrowableVec;
use crate::vm::{accessible_from, namespace_path, FileId, NamespaceId, VM};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstId(usize);

impl From<usize> for ConstId {
    fn from(data: usize) -> ConstId {
        ConstId(data)
    }
}

impl GrowableVec<RwLock<ConstData>> {
    pub fn idx(&self, index: ConstId) -> Arc<RwLock<ConstData>> {
        self.idx_usize(index.0 as usize)
    }
}

#[derive(Clone, Debug)]
pub struct ConstData {
    pub id: ConstId,
    pub file_id: FileId,
    pub ast: Arc<ast::Const>,
    pub namespace_id: NamespaceId,
    pub is_pub: bool,
    pub pos: Position,
    pub name: Name,
    pub ty: SourceType,
    pub expr: Box<ast::Expr>,
    pub value: ConstValue,
}

impl ConstData {
    pub fn name(&self, vm: &VM) -> String {
        namespace_path(vm, self.namespace_id, self.name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstValue {
    None,
    Bool(bool),
    Char(char),
    Int(i64),
    Float(f64),
}

impl ConstValue {
    pub fn to_bool(&self) -> bool {
        match self {
            &ConstValue::Bool(b) => b,
            _ => unreachable!(),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            &ConstValue::Char(c) => c,
            _ => unreachable!(),
        }
    }

    pub fn to_int(&self) -> i64 {
        match self {
            &ConstValue::Int(i) => i,
            _ => unreachable!(),
        }
    }

    pub fn to_float(&self) -> f64 {
        match self {
            &ConstValue::Float(f) => f,
            _ => unreachable!(),
        }
    }
}

pub fn const_accessible_from(vm: &VM, const_id: ConstId, namespace_id: NamespaceId) -> bool {
    let xconst = vm.consts.idx(const_id);
    let xconst = xconst.read();

    accessible_from(vm, xconst.namespace_id, xconst.is_pub, namespace_id)
}
