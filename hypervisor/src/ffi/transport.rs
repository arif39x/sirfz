use std::os::raw::c_int;

use super::loader::LoadedSymbols;
use super::ops;

pub struct TransportHandle {
    pub syms: LoadedSymbols,
    pub node_handle: c_int,
}

impl TransportHandle {
    pub fn load(lib_path: &str, is_server: bool, addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let syms = LoadedSymbols::open(lib_path)?;
        let node_handle = syms.start_node(is_server, addr)? as c_int;

        Ok(Self { syms, node_handle })
    }

    pub fn send(&self, data: &[u8]) -> i32 {
        ops::send(&self.syms, self.node_handle, data)
    }

    pub fn recv(&self, stream_idx: i32, buf: &mut [u8]) -> i32 {
        ops::recv(&self.syms, self.node_handle, stream_idx, buf)
    }
}

impl Drop for TransportHandle {
    fn drop(&mut self) {
        ops::stop(&self.syms, self.node_handle);
    }
}
