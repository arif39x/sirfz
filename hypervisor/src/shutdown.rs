use crate::hardening::memory;
use crate::secrets::secret;
use std::sync::Once;
use std::sync::atomic::{compiler_fence, Ordering};
use std::process;

static SHUTDOWN_ONCE: Once = Once::new();

pub fn secure_shutdown() {
    SHUTDOWN_ONCE.call_once(|| {
        eprintln!("\n[!] Secure shutdown initiated. Wiping memory...");

        secret::wipe_all_registered_secrets();

        compiler_fence(Ordering::SeqCst);

        memory::unlock_memory();

        process::exit(0);
    });
}
