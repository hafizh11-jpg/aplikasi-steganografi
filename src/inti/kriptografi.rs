// Modul kriptografi: derivasi kunci dan enkripsi/dekripsi AEAD.

use crate::inti::utilitas;
use crate::kesalahan::KesalahanAplikasi;
use argon2::Argon2;
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit},
};
use rand::Rng;
use zeroize::Zeroize;

/// Panjang salt dalam byte.
const PANJANG_SALT: usize = 16;
/// Panjang nonce dalam byte.
const PANJANG_NONCE: usize = 12;
/// Panjang kunci dalam byte.
const PANJANG_KUNCI: usize = 32;

/// Enkripsi data dengan kata sandi.
/// Output: salt(16) + nonce(12) + ciphertext.
pub fn enkripsi(data: &[u8], kata_sandi: &str) -> Result<Vec<u8>, KesalahanAplikasi> {
    utilitas::validasi_kata_sandi(kata_sandi)?;
    let mut salt_bytes = [0u8; PANJANG_SALT];
    rand::rng().fill_bytes(&mut salt_bytes);

    let mut kunci_bytes = [0u8; PANJANG_KUNCI];
    Argon2::default()
        .hash_password_into(kata_sandi.as_bytes(), &salt_bytes, &mut kunci_bytes)
        .map_err(|e| KesalahanAplikasi::Kriptografi(e.to_string()))?;

    let kunci = Key::clone_from_slice(&kunci_bytes);
    let cipher = ChaCha20Poly1305::new(&kunci);

    let mut nonce_bytes = [0u8; PANJANG_NONCE];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::clone_from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(&nonce, data)
        .map_err(|_| KesalahanAplikasi::Kriptografi("enkripsi gagal".into()))?;

    let mut out = Vec::with_capacity(PANJANG_SALT + PANJANG_NONCE + ciphertext.len());
    out.extend_from_slice(&salt_bytes);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);

    kunci_bytes.zeroize();
    Ok(out)
}

/// Dekripsi data dengan kata sandi.
pub fn dekripsi(data: &[u8], kata_sandi: &str) -> Result<Vec<u8>, KesalahanAplikasi> {
    utilitas::validasi_kata_sandi(kata_sandi)?;
    if data.len() < PANJANG_SALT + PANJANG_NONCE + 16 {
        return Err(KesalahanAplikasi::DekripsiGagal);
    }
    let salt_bytes = &data[..PANJANG_SALT];
    let nonce_bytes = &data[PANJANG_SALT..PANJANG_SALT + PANJANG_NONCE];
    let ciphertext = &data[PANJANG_SALT + PANJANG_NONCE..];

    let mut kunci_bytes = [0u8; PANJANG_KUNCI];
    Argon2::default()
        .hash_password_into(kata_sandi.as_bytes(), salt_bytes, &mut kunci_bytes)
        .map_err(|_| KesalahanAplikasi::DekripsiGagal)?;

    let kunci = Key::clone_from_slice(&kunci_bytes);
    let cipher = ChaCha20Poly1305::new(&kunci);
    let nonce = Nonce::clone_from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|_| KesalahanAplikasi::DekripsiGagal)?;

    kunci_bytes.zeroize();
    Ok(plaintext)
}

#[cfg(test)]
mod uji {
    use super::*;

    #[test]
    fn enkripsi_dekripsi_roundtrip() {
        let pesan = b"rahasia";
        let ct = enkripsi(pesan, "sandi123").unwrap();
        let pt = dekripsi(&ct, "sandi123").unwrap();
        assert_eq!(pt, pesan);
    }

    #[test]
    fn dekripsi_sandi_salah_gagal() {
        let ct = enkripsi(b"rahasia", "sandi123").unwrap();
        assert!(dekripsi(&ct, "salah").is_err());
    }
}
