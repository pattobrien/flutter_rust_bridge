use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub enum IrTypePrimitive {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Bool,
    Unit,
    Usize,
    Isize,
}
}

impl IrTypeTrait for IrTypePrimitive {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_file: &IrFile) {}
}
