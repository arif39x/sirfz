use libc::{mlockall, munlockall,MCL_CURRENT, MCL_FUTURE};

#[derive(Debug)]
pub enum SecurityError {
    MemoryLockFailed,
}

pub fn lock_memory() -> Result<(), SecurityError> {
    unsafe {
        if mlockall(MCL_CURRENT | MCL_FUTURE) != 0 {
            return Err(SecurityError::MemoryLockFailed);
        }
    }
    Ok(())
}

pub fn unlock_memory(){
    unsafe{
        let _ = munlockall();
    }
}