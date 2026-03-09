use std::os::raw::{c_char, c_int, c_uchar};

pub type FnStartNode   = unsafe extern "C" fn(c_int, *const c_char) -> c_int;
pub type FnSendMessage = unsafe extern "C" fn(c_int, *const c_uchar, c_int) -> c_int;
pub type FnRecvMessage = unsafe extern "C" fn(c_int, c_int, *mut c_uchar, c_int) -> c_int;
pub type FnStopNode    = unsafe extern "C" fn(c_int);
