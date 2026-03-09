use signal_hook::{consts::signal::*, iterator::Signals};
use std::thread;

use crate::shutdown::secure_shutdown;

pub fn install_signal_handlers() {
    let mut signals = Signals::new([SIGINT, SIGTERM])
        .expect("Failed to register signal handlers");

    thread::spawn(move || {
        for _ in signals.forever() {
            secure_shutdown();
        }
    });
}
