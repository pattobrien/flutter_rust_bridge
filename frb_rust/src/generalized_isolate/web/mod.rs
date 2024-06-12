mod into_dart;
mod misc;

use crate::for_generated::DartNativeSendPort;

pub use into_dart::*;
pub use misc::*;

pub type SendableDartSendPort = String;

#[derive(Debug, Clone)]
pub struct DartSendPort(web_sys::BroadcastChannel);

impl DartSendPort {
    pub fn new(native: DartNativeSendPort) -> Self {
        Self(web_sys::BroadcastChannel::new(&native).unwrap())
    }

    pub fn to_sendable(&self) -> SendableDartSendPort {
        self.0.name()
    }

    pub fn from_sendable(port: &SendableDartSendPort) -> DartSendPort {
        Self(web_sys::BroadcastChannel::new(port).unwrap())
    }

    pub fn post(&self, msg: impl IntoDart) -> bool {
        (self.0.post_message(&msg.into_dart()))
            .map_err(|err| crate::console_error!("error when DartSendPort.post: {:?}", err))
            .is_ok()
    }
}
