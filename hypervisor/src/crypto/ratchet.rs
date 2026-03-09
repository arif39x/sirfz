use super::kdf::derive_message_and_chain_keys;
use super::cipher;

pub struct RatchetState {
    chain_key:  [u8; 32],
    send_count: u64,
    recv_count: u64,
}

impl RatchetState {
    pub fn new(root_key: [u8; 32]) -> Self {
        Self { chain_key: root_key, send_count: 0, recv_count: 0 }
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let (message_key, next_chain_key) = derive_message_and_chain_keys(&self.chain_key);
        self.chain_key = next_chain_key;
        let ct = cipher::seal(&message_key, self.send_count, plaintext);
        self.send_count += 1;
        ct
    }

    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, &'static str> {
        let (message_key, next_chain_key) = derive_message_and_chain_keys(&self.chain_key);
        self.chain_key = next_chain_key;
        let pt = cipher::open(&message_key, ciphertext)?;
        self.recv_count += 1;
        Ok(pt)
    }
}
