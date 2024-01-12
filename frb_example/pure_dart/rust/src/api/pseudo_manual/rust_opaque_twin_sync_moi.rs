// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

pub use crate::auxiliary::sample_types::{HideData, NonCloneData, NonSendHideData};
use anyhow::Result;
use flutter_rust_bridge::{opaque_dyn, RustOpaque};
use std::fmt::Debug;
use std::ops::Deref;
pub use std::sync::{Mutex, RwLock};

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinSyncMoi;

/// Opaque types
pub trait DartDebugTwinSyncMoi: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinSyncMoi for T {}

pub enum EnumOpaqueTwinSyncMoi {
    Struct(RustOpaque<HideData>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebugTwinSyncMoi>>),
    Mutex(RustOpaque<Mutex<HideData>>),
    RwLock(RustOpaque<RwLock<HideData>>),
}

/// [`HideData`] has private fields.
pub struct OpaqueNestedTwinSyncMoi {
    pub first: RustOpaque<HideData>,
    pub second: RustOpaque<HideData>,
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_opaque_twin_sync_moi() -> RustOpaque<HideData> {
    RustOpaque::new(HideData::new())
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_option_opaque_twin_sync_moi(
    opaque: Option<RustOpaque<HideData>>,
) -> Option<RustOpaque<HideData>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_opaque_twin_sync_moi() -> SyncReturn<RustOpaque<HideData>> {
//     SyncReturn(RustOpaque::new(HideData::new()))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_array_opaque_enum_twin_sync_moi() -> [EnumOpaqueTwinSyncMoi; 5] {
    [
        EnumOpaqueTwinSyncMoi::Struct(RustOpaque::new(HideData::new())),
        EnumOpaqueTwinSyncMoi::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinSyncMoi::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSyncMoi::Mutex(RustOpaque::new(Mutex::new(HideData::new()))),
        EnumOpaqueTwinSyncMoi::RwLock(RustOpaque::new(RwLock::new(HideData::new()))),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_enum_opaque_twin_sync_moi(opaque: EnumOpaqueTwinSyncMoi) -> String {
    match opaque {
        EnumOpaqueTwinSyncMoi::Struct(s) => s.hide_data(),
        EnumOpaqueTwinSyncMoi::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSyncMoi::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSyncMoi::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaqueTwinSyncMoi::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_twin_sync_moi(opaque: RustOpaque<HideData>) -> String {
    opaque.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_opaque_with_delay_twin_sync_moi(opaque: RustOpaque<HideData>) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_twin_sync_moi() -> [RustOpaque<HideData>; 2] {
    [
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_non_clone_twin_sync_moi() -> SyncReturn<RustOpaque<NonCloneData>> {
//     SyncReturn(RustOpaque::new(NonCloneData::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_non_clone_twin_sync_moi(clone: RustOpaque<NonCloneData>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_sync_opaque_twin_sync_moi() -> RustOpaque<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

// TODO about sync
// #[flutter_rust_bridge::frb(rust_opaque_codec_moi)] #[flutter_rust_bridge::frb(sync)] pub fn sync_create_sync_opaque_twin_sync_moi() -> SyncReturn<RustOpaque<NonSendHideData>> {
//     SyncReturn(RustOpaque::new(NonSendHideData::new()))
// }

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_array_run_twin_sync_moi(data: [RustOpaque<HideData>; 2]) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_twin_sync_moi() -> Vec<RustOpaque<HideData>> {
    vec![
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn opaque_vec_run_twin_sync_moi(data: Vec<RustOpaque<HideData>>) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn create_nested_opaque_twin_sync_moi() -> OpaqueNestedTwinSyncMoi {
    OpaqueNestedTwinSyncMoi {
        first: RustOpaque::new(HideData::new()),
        second: RustOpaque::new(HideData::new()),
    }
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn run_nested_opaque_twin_sync_moi(opaque: OpaqueNestedTwinSyncMoi) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn unwrap_rust_opaque_twin_sync_moi(opaque: RustOpaque<HideData>) -> Result<String> {
    let data: HideData = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(rust_opaque_codec_moi)]
#[flutter_rust_bridge::frb(sync)]
pub fn frb_generator_test_twin_sync_moi() -> RustOpaque<FrbOpaqueReturnTwinSyncMoi> {
    panic!("dummy code");
}
