use crate::model::Envelope;
use crate::crypto::derive_shared_key;
use base64::{engine::general_purpose, Engine as _};

use aes::Aes256;
use ctr::Ctr128BE; // ✅ Este viene del crate 'ctr'
use cipher::{KeyIvInit, StreamCipher}; // ✅ Necesario para usar .apply_keystream()

use rand::rngs::OsRng;
use rand::RngCore;

use lzma_rs::{lzma_compress, lzma_decompress};

use std::fs::File;
use std::io::{Read, Write};

type AesCtr = Ctr128BE<Aes256>;

pub fn encode_file(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    let mut input = File::open(input_path)?;
    let mut json = String::new();
    input.read_to_string(&mut json)?;

    let mut compressed = Vec::new();
    lzma_compress(&mut json.as_bytes(), &mut compressed)?;

    let mut nonce = [0u8; 16];
    OsRng.fill_bytes(&mut nonce);

    let shared_key = [0x11; 32]; // Ejemplo, deberías derivar con derive_shared_key()

    let mut cipher = AesCtr::new(&shared_key.into(), &nonce.into());
    let mut ciphertext = compressed.clone();
    cipher.apply_keystream(&mut ciphertext);

    let envelope = Envelope {
        protocol: "cartoquantum".to_string(),
        version: 1,
        encoding: "lzma+aes256+x25519".to_string(),
        pk_sender: hex::encode([0u8;32]),
        pk_recipient: hex::encode([0u8;32]),
        issued_at: 0,
        sig: None,
        payload: general_purpose::STANDARD.encode([&nonce[..], &ciphertext[..]].concat()),
    };

    let mut out = File::create(output_path)?;
    let encoded = serde_json::to_string_pretty(&envelope)?;
    out.write_all(encoded.as_bytes())?;

    Ok(())
}

pub fn decode_file(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    let mut input = File::open(input_path)?;
    let mut encoded = String::new();
    input.read_to_string(&mut encoded)?;
    let shared_key = [0x11; 32];

    let envelope: Envelope = serde_json::from_str(&encoded)?;
    let data = general_purpose::STANDARD.decode(&envelope.payload)?;

    let (nonce, ciphertext) = data.split_at(16);

    let mut cipher = AesCtr::new(&shared_key.into(), nonce.into());
    let mut decrypted = ciphertext.to_vec();
    cipher.apply_keystream(&mut decrypted);

    let mut decompressed = Vec::new();
    lzma_decompress(&mut decrypted.as_slice(), &mut decompressed)?;

    let mut out = File::create(output_path)?;
    out.write_all(&decompressed)?;

    Ok(())
}
