use libc::{
    mount, umount2, unshare, CLONE_NEWNS, CLONE_NEWUSER, MNT_DETACH,
    MS_NOSUID, MS_NOEXEC, MS_NODEV, MS_PRIVATE, MS_REC,
};
use std::ffi::CString;
use std::fs;
use std::process;
use std::ptr;

pub fn isolate_environment() {
    let uid = unsafe { libc::getuid() };
    let gid = unsafe { libc::getgid() };

    unsafe {
        if unshare(CLONE_NEWUSER | CLONE_NEWNS) != 0 {
            process::exit(1);
        }

        fs::write("/proc/self/setgroups", b"deny").ok();

        let mut uid_buf = [0u8; 64];
        let mut uid_cursor = std::io::Cursor::new(&mut uid_buf[..]);
        std::io::Write::write_fmt(&mut uid_cursor, format_args!("0 {} 1", uid)).ok();
        let uid_len = uid_cursor.position() as usize;
        fs::write("/proc/self/uid_map", &uid_buf[..uid_len]).ok();

        let mut gid_buf = [0u8; 64];
        let mut gid_cursor = std::io::Cursor::new(&mut gid_buf[..]);
        std::io::Write::write_fmt(&mut gid_cursor, format_args!("0 {} 1", gid)).ok();
        let gid_len = gid_cursor.position() as usize;
        fs::write("/proc/self/gid_map", &gid_buf[..gid_len]).ok();

        let root = CString::new("/").unwrap();
        if mount(
            ptr::null(),
            root.as_ptr(),
            ptr::null(),
            MS_PRIVATE | MS_REC,
            ptr::null(),
        ) != 0
        {
            process::exit(1);
        }

        let tmp_path = CString::new("/tmp").unwrap();
        let tmpfs = CString::new("tmpfs").unwrap();
        let flags = MS_NOSUID | MS_NOEXEC | MS_NODEV;

        mount(
            tmpfs.as_ptr(),
            tmp_path.as_ptr(),
            tmpfs.as_ptr(),
            flags,
            ptr::null(),
        );

        let proc_path = CString::new("/proc").unwrap();
        let proc_fs = CString::new("proc").unwrap();

        umount2(proc_path.as_ptr(), MNT_DETACH);
        mount(
            proc_fs.as_ptr(),
            proc_path.as_ptr(),
            proc_fs.as_ptr(),
            flags,
            ptr::null(),
        );
    }
}