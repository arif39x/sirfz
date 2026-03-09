use libloading::{Library, Symbol};
use std::ffi::CString;

use super::types::{FnStartNode, FnSendMessage, FnRecvMessage, FnStopNode};

pub struct LoadedSymbols {
    pub _lib: Library,
    pub start_node:   FnStartNode,
    pub send_message: FnSendMessage,
    pub recv_message: FnRecvMessage,
    pub stop_node:    FnStopNode,
}

impl LoadedSymbols {
    pub fn open(lib_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let lib = unsafe { Library::new(lib_path)? };

        let start_node: FnStartNode = unsafe {
            let sym: Symbol<FnStartNode> = lib.get(b"StartNode\0")?;
            *sym
        };
        let send_message: FnSendMessage = unsafe {
            let sym: Symbol<FnSendMessage> = lib.get(b"SendMessage\0")?;
            *sym
        };
        let recv_message: FnRecvMessage = unsafe {
            let sym: Symbol<FnRecvMessage> = lib.get(b"RecvMessage\0")?;
            *sym
        };
        let stop_node: FnStopNode = unsafe {
            let sym: Symbol<FnStopNode> = lib.get(b"StopNode\0")?;
            *sym
        };

        Ok(Self { _lib: lib, start_node, send_message, recv_message, stop_node })
    }

    pub fn start_node(&self, is_server: bool, addr: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let c_addr = CString::new(addr)?;
        let handle = unsafe {
            (self.start_node)(if is_server { 1 } else { 0 }, c_addr.as_ptr())
        };
        if handle < 0 {
            return Err("StartNode returned error".into());
        }
        Ok(handle)
    }
}
