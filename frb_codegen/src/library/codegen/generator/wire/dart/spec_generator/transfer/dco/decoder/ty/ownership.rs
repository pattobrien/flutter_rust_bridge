use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for OwnershipWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        "".to_string()
    }
}
