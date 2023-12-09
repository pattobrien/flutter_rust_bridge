// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 1.82.4.

#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]

// Section: imports

use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::{transform_result_dco, SseSerializer};
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: executor

#[cfg(not(target_family = "wasm"))]
flutter_rust_bridge::for_generated::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER:
    flutter_rust_bridge::DefaultHandler<flutter_rust_bridge::for_generated::SimpleThreadPool>
    = flutter_rust_bridge::DefaultHandler::new_simple(Default::default());
}

#[cfg(target_family = "wasm")]
thread_local! {
    pub static THREAD_POOL: flutter_rust_bridge::for_generated::SimpleThreadPool = Default::default();
}

#[cfg(target_family = "wasm")]
flutter_rust_bridge::for_generated::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER:
    flutter_rust_bridge::DefaultHandler<&'static std::thread::LocalKey<flutter_rust_bridge::for_generated::SimpleThreadPool>>
    = flutter_rust_bridge::DefaultHandler::new_simple(&THREAD_POOL);
}

// Section: wire_funcs

fn wire_hello_impl(port_: i64, ptr_: *mut u8, rust_vec_len_: i32, data_len_: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "hello",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let mut deserializer = unsafe {
                flutter_rust_bridge::for_generated::SseDeserializer::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let api_a = i32::sse_decode(&mut deserializer);
            let api_b = i32::sse_decode(&mut deserializer);
            deserializer.end();
            move |context| {
                transform_result_sse(Result::<_, ()>::Ok(crate::api::minimal::hello(
                    api_a, api_b,
                )))
            }
        },
    )
}
fn wire_hello_sync_impl(
    ptr_: *mut u8,
    rust_vec_len_: i32,
    data_len_: i32,
) -> flutter_rust_bridge::for_generated::WireSyncReturnSse {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync::<flutter_rust_bridge::for_generated::SseCodec, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "hello_sync",
            port: None,
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Sync,
        },
        move || {
            let mut deserializer = unsafe {
                flutter_rust_bridge::for_generated::SseDeserializer::from_wire(
                    ptr_,
                    rust_vec_len_,
                    data_len_,
                )
            };
            let api_a = i32::sse_decode(&mut deserializer);
            let api_b = i32::sse_decode(&mut deserializer);
            deserializer.end();
            transform_result_sse(Result::<_, ()>::Ok(crate::api::minimal::hello_sync(
                api_a, api_b,
            )))
        },
    )
}
fn wire_minimal_adder_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    a: impl CstDecode<i32> + core::panic::UnwindSafe,
    b: impl CstDecode<i32> + core::panic::UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "minimal_adder",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_a = a.cst_decode();
            let api_b = b.cst_decode();
            move |context| {
                transform_result_dco(Result::<_, ()>::Ok(crate::api::minimal::minimal_adder(
                    api_a, api_b,
                )))
            }
        },
    )
}

// Section: dart2rust

pub trait CstDecode<T> {
    fn cst_decode(self) -> T;
}

impl<T, S> CstDecode<Option<T>> for *mut S
where
    *mut S: CstDecode<T>,
{
    fn cst_decode(self) -> Option<T> {
        (!self.is_null()).then(|| self.cst_decode())
    }
}
impl CstDecode<i32> for i32 {
    fn cst_decode(self) -> i32 {
        self
    }
}
pub trait SseDecode {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self;
}

impl SseDecode for i32 {
    fn sse_decode(deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer) -> Self {
        deserializer.cursor.read_i32::<NativeEndian>().unwrap()
    }
}

// Section: rust2dart

// TODO remove
fn transform_result_sse<T, E>(
    raw: Result<T, E>,
) -> Result<
    flutter_rust_bridge::for_generated::Rust2DartMessageSse,
    flutter_rust_bridge::for_generated::Rust2DartMessageSse,
>
where
    T: SseEncode,
    E: SseEncode,
{
    use flutter_rust_bridge::for_generated::{Rust2DartAction, SseCodec};

    match raw {
        Ok(raw) => Ok(SseCodec::encode(Rust2DartAction::Success, |serializer| {
            raw.sse_encode(serializer)
        })),
        Err(raw) => Err(SseCodec::encode(Rust2DartAction::Error, |serializer| {
            raw.sse_encode(serializer)
        })),
    }
}

// TODO put it there
impl SseEncode for () {
    fn sse_encode(self, serializer: &mut SseSerializer) {}
}

pub trait SseEncode {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer);
}

// TODO

impl SseEncode for i32 {
    fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer) {
        serializer.cursor.write_i32::<NativeEndian>(self).unwrap();
    }
}

#[cfg(not(target_family = "wasm"))]
#[path = "frb_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "frb_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;
