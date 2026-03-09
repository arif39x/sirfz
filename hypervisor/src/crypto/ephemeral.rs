use ed25519_dalek::{SigningKey, VerifyingKey};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use rand::rngs::OsRng;

pub struct EphemeralIdentity {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub dh_secret: EphemeralSecret,
    pub dh_public: X25519PublicKey,
}

pub fn generate() -> EphemeralIdentity {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let dh_secret = EphemeralSecret::random_from_rng(OsRng);
    let dh_public = X25519PublicKey::from(&dh_secret);

    EphemeralIdentity {
        signing_key,
        verifying_key,
        dh_secret,
        dh_public,
    }
}
