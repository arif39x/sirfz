mod handshake;

use std::ffi::CStr;
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::os::raw::{c_char, c_int, c_uchar};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

use zeroize::Zeroize;

const BUF_SIZE: usize = 32 * 1024;
const MAX_HANDLES: usize = 64;

static HANDLES: Mutex<Vec<Option<Arc<Mutex<TcpStream>>>>> = Mutex::new(Vec::new());
static HANDLE_SEQ: AtomicI32 = AtomicI32::new(0);

fn alloc_handle(stream: TcpStream) -> c_int {
    let stream = Arc::new(Mutex::new(stream));
    let mut handles = HANDLES.lock().unwrap();
    let idx = HANDLE_SEQ.fetch_add(1, Ordering::Relaxed) as usize % MAX_HANDLES;
    if idx < handles.len() && handles[idx].is_some() {
        for i in 0..handles.len() {
            if handles[i].is_none() {
                handles[i] = Some(Arc::clone(&stream));
                return i as c_int;
            }
        }
    }
    if idx >= handles.len() {
        handles.resize_with(idx + 1, || None);
    }
    handles[idx] = Some(stream);
    idx as c_int
}

fn free_handle(handle: c_int) {
    if handle < 0 {
        return;
    }
    let mut handles = HANDLES.lock().unwrap();
    if let Some(slot) = handles.get_mut(handle as usize) {
        if let Some(stream) = slot.take() {
            if let Ok(s) = stream.lock() {
                let _ = s.shutdown(Shutdown::Both);
            }
        }
    }
}

fn with_stream<F, R>(handle: c_int, f: F) -> Option<R>
where
    F: FnOnce(&mut TcpStream) -> R,
{
    if handle < 0 {
        return None;
    }
    let handles = HANDLES.lock().ok()?;
    let slot = handles.get(handle as usize)?;
    let stream = slot.as_ref()?;
    let mut guard = stream.lock().ok()?;
    Some(f(&mut *guard))
}

fn send_frame(stream: &mut TcpStream, data: &[u8]) -> io::Result<()> {
    let len = data.len();
    if len > u32::MAX as usize {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "message too large"));
    }
    let len_bytes = (len as u32).to_be_bytes();
    stream.write_all(&len_bytes)?;
    stream.write_all(data)?;
    stream.flush()?;
    Ok(())
}

fn recv_frame(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = u32::from_be_bytes(len_buf) as usize;
    if len > BUF_SIZE {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "frame too large"));
    }
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    Ok(buf)
}

unsafe fn cstr_to_str<'a>(ptr: *const c_char) -> &'a str {
    CStr::from_ptr(ptr).to_str().unwrap_or("0.0.0.0:9000")
}

unsafe fn slice_from_ptr<'a>(ptr: *const c_uchar, len: usize) -> &'a [u8] {
    std::slice::from_raw_parts(ptr, len)
}

unsafe fn slice_from_ptr_mut<'a>(ptr: *mut c_uchar, len: usize) -> &'a mut [u8] {
    std::slice::from_raw_parts_mut(ptr, len)
}

#[no_mangle]
pub unsafe extern "C" fn StartNode(
    is_server: c_int,
    addr: *const c_char,
    auth_key: *const c_uchar,
) -> c_int {
    let addr = cstr_to_str(addr);
    let auth_key = slice_from_ptr(auth_key, 32);
    let mut key = [0u8; 32];
    key.copy_from_slice(auth_key);

    let result = if is_server != 0 {
        start_server(addr, &key)
    } else {
        start_client(addr, &key)
    };

    key.zeroize();

    match result {
        Ok(handle) => handle,
        Err(_) => -1,
    }
}

fn start_server(addr: &str, auth_key: &[u8; 32]) -> io::Result<c_int> {
    let listener = TcpListener::bind(addr)?;
    listener.set_nonblocking(false)?;

    let (mut stream, _) = listener.accept()?;
    handshake::server(&mut stream, auth_key)?;

    Ok(alloc_handle(stream))
}

fn start_client(addr: &str, auth_key: &[u8; 32]) -> io::Result<c_int> {
    let mut stream = TcpStream::connect(addr)?;
    handshake::client(&mut stream, auth_key)?;

    Ok(alloc_handle(stream))
}

#[no_mangle]
pub unsafe extern "C" fn SendMessage(
    handle: c_int,
    data: *const c_uchar,
    length: c_int,
) -> c_int {
    if length < 0 {
        return -1;
    }
    let buf = slice_from_ptr(data, length as usize);
    let ok = with_stream(handle, |stream| send_frame(stream, buf).is_ok());
    match ok {
        Some(true) => length,
        _ => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn RecvMessage(
    handle: c_int,
    _stream_idx: c_int,
    out_buf: *mut c_uchar,
    out_len: c_int,
) -> c_int {
    if out_len < 0 {
        return -1;
    }
    let dst = slice_from_ptr_mut(out_buf, out_len as usize);

    let result = with_stream(handle, |stream| recv_frame(stream));
    match result {
        Some(Ok(mut frame)) => {
            let n = frame.len().min(dst.len());
            dst[..n].copy_from_slice(&frame[..n]);
            frame.zeroize();
            n as c_int
        }
        _ => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn StopNode(handle: c_int) {
    free_handle(handle);
}
