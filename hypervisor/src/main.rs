mod hardening;
mod secrets;
mod shutdown;
mod ffi;
mod crypto;
mod chat;

use hardening::{memory, dump, signals, anti_debug, seccomp, namespace, watchdog};
use crypto::ratchet::RatchetState;
use chat::router::HostRouter;
use ffi::transport::TransportHandle;
use std::env;
use std::sync::{Arc, Mutex};
use std::panic;
use obfstr::obfstr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    panic::set_hook(Box::new(|_| {
        eprintln!("\n{}", obfstr!("[CRITICAL] Panic detected! Executing emergency wipe."));
        shutdown::secure_shutdown();
    }));

    anti_debug::block_debugger();
    dump::disable_core_dumps();

    watchdog::secure_launch(|| {
        namespace::isolate_environment();
        memory::lock_memory().map_err(|e| format!("{:?}", e))?;
        seccomp::enforce_syscall_boundaries();
        signals::install_signal_handlers();

        let _identity = crypto::ephemeral::generate();

        let args: Vec<String> = env::args().collect();
        let is_server = args.iter().any(|a| a == "--server");
        let addr = args.iter()
            .position(|a| a == "--addr")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.as_str())
            .unwrap_or("0.0.0.0:9000");

        let lib_path = args.iter()
            .position(|a| a == "--lib")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.as_str())
            .unwrap_or("./libtransport.so");


        eprintln!("\x1b[1;30m[+]\x1b[0m \x1b[32m{}\x1b[0m", obfstr!("SECURE ENVIRONMENT ACTIVE"));
        eprintln!("\x1b[1;30m[+]\x1b[0m \x1b[32mTransport: {} | Mode: {}\x1b[0m\n", addr, if is_server { "SERVER" } else { "CLIENT" });

        let transport = TransportHandle::load(lib_path, is_server, addr)
            .map_err(|e| format!("transport init failed: {}", e))?;

        let root_key = [0x42u8; 32];
        let send_ratchet = RatchetState::new(root_key);
        let recv_ratchet = Arc::new(Mutex::new(RatchetState::new(root_key)));

        let router = HostRouter::new(transport);
        chat::terminal::run(router, send_ratchet, recv_ratchet);

        Ok(())
    });

    Ok(())
}
