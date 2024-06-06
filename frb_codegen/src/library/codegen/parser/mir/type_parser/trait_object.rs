use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;
use anyhow::bail;
use syn::TypeTraitObject;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_trait_object(
        &mut self,
        _type_trait_object: &TypeTraitObject,
    ) -> anyhow::Result<MirType> {
        todo!()
    }
}
