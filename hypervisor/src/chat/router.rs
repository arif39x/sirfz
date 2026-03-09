use crate::ffi::transport::TransportHandle;
use std::sync::{Arc, RwLock};

pub struct HostRouter {
    transport: Arc<RwLock<TransportHandle>>,
}

impl HostRouter {
    pub fn new(transport: TransportHandle) -> Self {
        Self {
            transport: Arc::new(RwLock::new(transport)),
        }
    }

    pub fn fan_out(&self, ciphertext: &[u8]) {
        if let Ok(t) = self.transport.read() {
            t.send(ciphertext);
        }
    }

    pub fn poll_recv(&self, stream_idx: i32, buf: &mut [u8]) -> i32 {
        if let Ok(t) = self.transport.read() {
            return t.recv(stream_idx, buf);
        }
        -1
    }

    pub fn inner(&self) -> Arc<RwLock<TransportHandle>> {
        Arc::clone(&self.transport)
    }
}
