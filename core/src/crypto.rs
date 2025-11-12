use x25519_dalek::{EphemeralSecret, PublicKey};
use sha2::Sha256;
use hkdf::Hkdf;
use arrayref::array_ref;
use rand::rngs::OsRng;


pub fn derive_shared_key(my_secret: &[u8], their_public: &[u8]) -> [u8; 32] {
let my_secret = EphemeralSecret::random_from_rng(OsRng);
let their_public = PublicKey::from(*array_ref!(their_public, 0, 32));


    let shared = my_secret.diffie_hellman(&their_public);
    let hk = Hkdf::<Sha256>::new(None, shared.as_bytes());
    let mut okm = [0u8; 32];
    hk.expand(b"cartoquantum", &mut okm).unwrap();
    okm
}
