use libc::{prctl, PR_SET_DUMPABLE, setrlimit, rlimit, RLIMIT_CORE};

pub fn disable_core_dumps() {
    unsafe {
        if prctl(PR_SET_DUMPABLE, 0, 0, 0, 0) != 0 {    
            panic!("Failed to disable core dumps");
        }
        
        let limit = rlimit {
            rlim_cur: 0,   
            rlim_max: 0,   
        };
        
        if setrlimit(RLIMIT_CORE, &limit) != 0 {
            panic!("Failed to set core limit RLMIT_CORE");
        }
    }
}
