// Utilitas umum aplikasi.

use crate::kesalahan::KesalahanAplikasi;

/// Magic bytes untuk menandai data steganografi.
pub const MAGIC_STEGO: &[u8] = b"STEG";

/// Maksimum ukuran pesan (10 MB) untuk mencegah penyalahgunaan memori.
pub const BATAS_UKURAN_PESAN: usize = 10 * 1024 * 1024;

/// Mengubah byte menjadi vektor bit (MSB first).
pub fn byte_ke_bit(data: &[u8]) -> Vec<bool> {
    let mut bit = Vec::with_capacity(data.len() * 8);
    for &b in data {
        for i in (0..8).rev() {
            bit.push((b >> i) & 1 == 1);
        }
    }
    bit
}

/// Mengubah vektor bit menjadi byte.
pub fn bit_ke_byte(bit: &[bool]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bit.len() / 8);
    for chunk in bit.chunks(8) {
        if chunk.len() < 8 {
            break;
        }
        let mut b: u8 = 0;
        for &x in chunk {
            b = (b << 1) | u8::from(x);
        }
        out.push(b);
    }
    out
}

/// Menghitung checksum sederhana (CRC32) untuk verifikasi integritas.
pub fn hitung_checksum(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

/// Membuat header data: magic(4) + panjang(4) + checksum(4).
pub fn buat_header(panjang: usize, checksum: u32) -> Vec<u8> {
    let mut h = Vec::with_capacity(12);
    h.extend_from_slice(MAGIC_STEGO);
    h.extend_from_slice(&(panjang as u32).to_le_bytes());
    h.extend_from_slice(&checksum.to_le_bytes());
    h
}

/// Membaca header dan mengembalikan (panjang, checksum) jika valid.
pub fn baca_header(data: &[u8]) -> Result<(usize, u32), KesalahanAplikasi> {
    if data.len() < 12 {
        return Err(KesalahanAplikasi::DataTidakDitemukan);
    }
    if &data[0..4] != MAGIC_STEGO {
        return Err(KesalahanAplikasi::DataTidakDitemukan);
    }
    let panjang = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    let checksum = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    if panjang > BATAS_UKURAN_PESAN {
        return Err(KesalahanAplikasi::DataTidakDitemukan);
    }
    Ok((panjang, checksum))
}

/// Validasi kata sandi tidak kosong.
pub fn validasi_kata_sandi(kata_sandi: &str) -> Result<(), KesalahanAplikasi> {
    if kata_sandi.trim().is_empty() {
        return Err(KesalahanAplikasi::KataSandiKosong);
    }
    Ok(())
}

/// Validasi pesan tidak kosong.
pub fn validasi_pesan(pesan: &str) -> Result<(), KesalahanAplikasi> {
    if pesan.trim().is_empty() {
        return Err(KesalahanAplikasi::PesanKosong);
    }
    if pesan.len() > BATAS_UKURAN_PESAN {
        return Err(KesalahanAplikasi::KapasitasTidakCukup);
    }
    Ok(())
}

#[cfg(test)]
mod uji {
    use super::*;

    #[test]
    fn bit_roundtrip() {
        let data = b"halo";
        let bit = byte_ke_bit(data);
        let balik = bit_ke_byte(&bit);
        assert_eq!(balik, data);
    }

    #[test]
    fn header_roundtrip() {
        let h = buat_header(42, 12345);
        let (p, c) = baca_header(&h).unwrap();
        assert_eq!(p, 42);
        assert_eq!(c, 12345);
    }

    #[test]
    fn checksum_deterministik() {
        assert_eq!(hitung_checksum(b"abc"), hitung_checksum(b"abc"));
        assert_ne!(hitung_checksum(b"abc"), hitung_checksum(b"abd"));
    }

    #[test]
    fn validasi_kata_sandi_kosong() {
        assert!(validasi_kata_sandi("").is_err());
        assert!(validasi_kata_sandi("x").is_ok());
    }
}
