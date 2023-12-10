use crate::generalized_isolate::PortLike;
use wasm_bindgen::JsCast;
use web_sys::BroadcastChannel;

/// cbindgen:ignore
pub type WireSyncReturnDco = wasm_bindgen::JsValue;
/// cbindgen:ignore
pub type WireSyncReturnSse = wasm_bindgen::JsValue;

pub type DartAbi = wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct SendableMessagePortHandle(String);

pub fn message_port_to_handle(port: &MessagePort) -> SendableMessagePortHandle {
    SendableMessagePortHandle(
        port.dyn_ref::<BroadcastChannel>()
            .map(|channel| channel.name())
            .expect("Not a BroadcastChannel"),
    )
}

pub fn handle_to_message_port(handle: &SendableMessagePortHandle) -> MessagePort {
    PortLike::broadcast(&handle.0)
}
