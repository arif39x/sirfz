use std::fs;
use std::process;

pub fn block_debugger() {
    let status = fs::read_to_string("/proc/self/status").unwrap_or_default();
    for line in status.lines() {
        if line.starts_with("TracerPid:") {
            let tracer_pid: i64 = line
                .split_whitespace()
                .nth(1)
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            if tracer_pid != 0 {
                process::exit(1);
            }
            return;
        }
    }
}
