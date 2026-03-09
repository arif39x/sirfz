use std::os::raw::{c_int, c_uchar};

use super::loader::LoadedSymbols;

pub fn send(syms: &LoadedSymbols, node_handle: c_int, data: &[u8]) -> i32 {
    unsafe {
        (syms.send_message)(
            node_handle,
            data.as_ptr() as *const c_uchar,
            data.len() as c_int,
        )
    }
}

pub fn recv(syms: &LoadedSymbols, node_handle: c_int, stream_idx: i32, buf: &mut [u8]) -> i32 {
    unsafe {
        (syms.recv_message)(
            node_handle,
            stream_idx as c_int,
            buf.as_mut_ptr() as *mut c_uchar,
            buf.len() as c_int,
        )
    }
}

pub fn stop(syms: &LoadedSymbols, node_handle: c_int) {
    unsafe { (syms.stop_node)(node_handle) }
}
