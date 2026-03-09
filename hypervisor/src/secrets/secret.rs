use secrecy::{Secret, ExposeSecret};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

const MAX_SECRETS: usize = 64;
const INIT_PTR: AtomicPtr<Secret<[u8; 32]>> = AtomicPtr::new(ptr::null_mut());
static SECRET_REGISTRY: [AtomicPtr<Secret<[u8; 32]>>; MAX_SECRETS] = [INIT_PTR; MAX_SECRETS];

pub struct SecureSecret {
    #[allow(dead_code)]
    inner: Box<Secret<[u8; 32]>>,
    registry_index: Option<usize>,
}

impl SecureSecret {
    pub fn new(initial_data: [u8; 32]) -> Self {
        let mut inner = Box::new(Secret::new(initial_data));
        let ptr: *mut Secret<[u8; 32]> = &mut *inner;
        
        let mut registry_index = None;
        for (i, atomic_ptr) in SECRET_REGISTRY.iter().enumerate() {
            if atomic_ptr.compare_exchange(ptr::null_mut(), ptr, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                registry_index = Some(i);
                break;
            }
        }

        Self { inner, registry_index }
    }

    #[allow(dead_code)]
    pub fn expose(&self) -> [u8; 32] {
        *self.inner.expose_secret()
    }
}

impl Drop for SecureSecret {
    fn drop(&mut self) {
        if let Some(idx) = self.registry_index {
            SECRET_REGISTRY[idx].store(ptr::null_mut(), Ordering::SeqCst);
        }
    }
}

pub fn wipe_all_registered_secrets() {
    for atomic_ptr in &SECRET_REGISTRY {
        let ptr = atomic_ptr.swap(ptr::null_mut(), Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                *ptr = Secret::new([0u8; 32]);
            }
        }
    }
}
