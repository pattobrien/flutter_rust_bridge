// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.16.

// Section: imports

use super::*;
use crate::api::minimal::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::for_generated::wasm_bindgen;
use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: dart2rust

impl<T> CstDecode<Option<T>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
where
    JsValue: CstDecode<T>,
{
    fn cst_decode(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.cst_decode())
    }
}
impl
    CstDecode<
        flutter_rust_bridge::RustOpaqueMoi<
            flutter_rust_bridge::for_generated::rust_async::RwLock<Apple>,
        >,
    > for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(
        self,
    ) -> flutter_rust_bridge::RustOpaqueMoi<
        flutter_rust_bridge::for_generated::rust_async::RwLock<Apple>,
    > {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }
        decode_rust_opaque_moi((self.as_f64().unwrap() as usize) as _)
    }
}
impl CstDecode<i32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<usize> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> usize {
        self.unchecked_into_f64() as _
    }
}

#[wasm_bindgen]
pub fn dart_fn_deliver_output(
    call_id: i32,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    let message = unsafe {
        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
            ptr_,
            rust_vec_len_,
            data_len_,
        )
    };
    FLUTTER_RUST_BRIDGE_HANDLER.dart_fn_handle_output(call_id, message)
}

#[wasm_bindgen]
pub fn wire_Apple_apple_method_ref(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_Apple_apple_method_ref_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_Apple_new(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_Apple_new_impl(port_)
}

#[wasm_bindgen]
pub fn wire_init_app(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_init_app_impl(port_)
}

#[wasm_bindgen]
pub fn wire_minimal_adder(port_: flutter_rust_bridge::for_generated::MessagePort, a: i32, b: i32) {
    wire_minimal_adder_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApple(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Apple>>::increment_strong_count(
        ptr as _,
    );
}

#[wasm_bindgen]
pub fn rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockApple(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Apple>>::decrement_strong_count(
        ptr as _,
    );
}
