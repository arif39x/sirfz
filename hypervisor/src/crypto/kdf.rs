use hkdf::Hkdf;
use sha2::Sha256;

pub fn derive_message_and_chain_keys(chain_key: &[u8; 32]) -> ([u8; 32], [u8; 32]) {
    let hk = Hkdf::<Sha256>::new(None, chain_key);
    let mut message_key = [0u8; 32];
    let mut next_chain_key = [0u8; 32];
    hk.expand(b"msg", &mut message_key).unwrap();
    hk.expand(b"chain", &mut next_chain_key).unwrap();
    (message_key, next_chain_key)
}
