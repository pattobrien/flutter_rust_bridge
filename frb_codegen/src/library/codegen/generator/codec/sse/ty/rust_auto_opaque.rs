use crate::codegen::generator::codec::sse::ty::delegate::{
    simple_delegate_decode, simple_delegate_encode,
};
use crate::codegen::generator::codec::sse::ty::rust_opaque::generate_generalized_rust_opaque_encode;
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for RustAutoOpaqueCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let needs_move = self.ir.needs_move();
        Some(generate_generalized_rust_opaque_encode(
            lang,
            &format!("{needs_move}"),
        ))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(simple_delegate_decode(
            lang,
            &IrTypeRustOpaque::DELEGATE_TYPE,
            "inner",
        ))
    }
}
