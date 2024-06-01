use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::rust_opaque::RustOpaqueCodecMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserInternalConfig {
    pub rust_input_namespace_pack: RustInputNamespacePack,
    pub rust_crate_dir: PathBuf,
    pub force_codec_mode_pack: Option<CodecModePack>,
    pub default_stream_sink_codec: CodecMode,
    pub default_rust_opaque_codec: RustOpaqueCodecMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RustInputNamespacePack {
    rust_input_namespace_prefices: Vec<Namespace>,
}

impl RustInputNamespacePack {
    pub fn is_interest(&self, namespace: &Namespace) -> bool {
        (self.rust_input_namespace_prefices.iter()).any(|prefix| prefix.is_prefix_of(namespace))
    }
}
