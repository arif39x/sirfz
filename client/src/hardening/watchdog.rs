use libc::{
    fork, kill, ptrace, waitpid, PTRACE_CONT, PTRACE_TRACEME, SIGKILL,
    WEXITSTATUS, WIFEXITED, WIFSIGNALED, WTERMSIG,
};
use std::process;

pub fn secure_launch<F>(sandbox_payload: F)
where
    F: FnOnce() -> Result<(), Box<dyn std::error::Error>>,
{
    unsafe {
        let child_process_id = fork();

        if child_process_id < 0 {
            process::exit(1);
        }

        if child_process_id == 0 {
            if ptrace(PTRACE_TRACEME, 0, 0, 0) < 0 {
                process::exit(1);
            }

            if sandbox_payload().is_err() {
                process::exit(1);
            }

            process::exit(0);
        } else {
            loop {
                let mut child_execution_status = 0;
                waitpid(child_process_id, &mut child_execution_status, 0);

                if WIFEXITED(child_execution_status) {
                    process::exit(WEXITSTATUS(child_execution_status));
                }

                if WIFSIGNALED(child_execution_status) {
                    eprintln!(
                        "\n[!] Watchdog Alert: Child terminated by signal {}",
                        WTERMSIG(child_execution_status)
                    );
                    process::exit(1);
                }

                if ptrace(PTRACE_CONT, child_process_id, 0, 0) < 0 {
                    kill(child_process_id, SIGKILL);
                    process::exit(1);
                }
            }
        }
    }
}
