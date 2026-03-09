use crate::crypto::ratchet::RatchetState;
use crate::chat::router::HostRouter;
use crate::hardening::input::read_line_raw;

pub fn run(router: &HostRouter, send_ratchet: &mut RatchetState) {
    loop {
        use std::io::{Write, stdout};
        print!("\x1b[1;31m[\x1b[0m\x1b[1;35mroot@sirfz\x1b[0m\x1b[1;31m]\x1b[0m\x1b[1;34m~\x1b[0m\x1b[1;31m#\x1b[0m ");
        stdout().flush().unwrap();
        let line = read_line_raw();

        if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
            crate::shutdown::secure_shutdown();
            break;
        }

        if line.is_empty() {
            continue;
        }

        let ciphertext = send_ratchet.encrypt(line.as_bytes());
        router.fan_out(&ciphertext);
    }
}
