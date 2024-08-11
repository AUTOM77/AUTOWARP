use ring::{agreement, rand};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};

pub fn get_key() -> String {
    let rng = rand::SystemRandom::new();
    let private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
    let public_key = private_key.compute_public_key().unwrap();

    let res = BASE64.encode(&public_key);
    res
}

