// Steganografi pada gambar (LSB pada kanal RGB).

use crate::inti::kriptografi;
use crate::inti::utilitas;
use crate::kesalahan::KesalahanAplikasi;
use image::{GenericImageView, RgbaImage};

/// Menyisipkan pesan terenkripsi ke dalam gambar.
pub fn sisipkan_pesan_gambar(
    jalur_masukan: &str,
    jalur_keluaran: &str,
    pesan: &str,
    kata_sandi: &str,
) -> Result<(), KesalahanAplikasi> {
    utilitas::validasi_pesan(pesan)?;
    utilitas::validasi_kata_sandi(kata_sandi)?;

    let img = image::open(jalur_masukan).map_err(KesalahanAplikasi::Gambar)?;
    let (lebar, tinggi) = img.dimensions();
    let mut rgba: RgbaImage = img.to_rgba8();

    let data_terenkripsi = kriptografi::enkripsi(pesan.as_bytes(), kata_sandi)?;
    let checksum = utilitas::hitung_checksum(&data_terenkripsi);
    let mut payload = utilitas::buat_header(data_terenkripsi.len(), checksum);
    payload.extend_from_slice(&data_terenkripsi);

    let bit = utilitas::byte_ke_bit(&payload);
    let kapasitas = (lebar as usize) * (tinggi as usize) * 3;
    if bit.len() > kapasitas {
        return Err(KesalahanAplikasi::KapasitasTidakCukup);
    }

    let mut indeks_bit = 0usize;
    'outer: for y in 0..tinggi {
        for x in 0..lebar {
            let pixel = rgba.get_pixel_mut(x, y);
            for kanal in 0..3usize {
                if indeks_bit >= bit.len() {
                    break 'outer;
                }
                let nilai = pixel[kanal];
                pixel[kanal] = (nilai & 0xFE) | u8::from(bit[indeks_bit]);
                indeks_bit += 1;
            }
            // alpha tidak diubah
        }
    }

    rgba.save(jalur_keluaran)
        .map_err(|e| KesalahanAplikasi::GagalMenulisBerkas(e.to_string()))?;
    Ok(())
}

/// Membaca pesan tersembunyi dari gambar.
pub fn baca_pesan_gambar(
    jalur_masukan: &str,
    kata_sandi: &str,
) -> Result<String, KesalahanAplikasi> {
    utilitas::validasi_kata_sandi(kata_sandi)?;
    let img = image::open(jalur_masukan).map_err(KesalahanAplikasi::Gambar)?;
    let (lebar, tinggi) = img.dimensions();
    let rgba = img.to_rgba8();

    // Baca seluruh bit kanal RGB
    let mut bit: Vec<bool> = Vec::with_capacity((lebar * tinggi * 3) as usize);
    for y in 0..tinggi {
        for x in 0..lebar {
            let p = rgba.get_pixel(x, y);
            for kanal in 0..3usize {
                bit.push(p[kanal] & 1 == 1);
            }
        }
    }

    // Header 12 byte = 96 bit
    if bit.len() < 96 {
        return Err(KesalahanAplikasi::DataTidakDitemukan);
    }
    let header_byte = utilitas::bit_ke_byte(&bit[..96]);
    let (panjang, checksum) = utilitas::baca_header(&header_byte)?;

    let total_bit = (12 + panjang) * 8;
    if bit.len() < total_bit {
        return Err(KesalahanAplikasi::DataTidakDitemukan);
    }
    let payload_bit = &bit[96..total_bit];
    let data_terenkripsi = utilitas::bit_ke_byte(payload_bit);

    // Verifikasi checksum
    if utilitas::hitung_checksum(&data_terenkripsi) != checksum {
        return Err(KesalahanAplikasi::ChecksumTidakCocok);
    }

    let plaintext = kriptografi::dekripsi(&data_terenkripsi, kata_sandi)?;
    String::from_utf8(plaintext)
        .map_err(|_| KesalahanAplikasi::DekripsiGagal)
}


#[cfg(test)]
mod uji {
    use super::*;
    use image::{ImageBuffer, Rgba};
    use std::env::temp_dir;

    #[test]
    fn roundtrip_gambar() {
        let dir = temp_dir();
        let masukan = dir.join("in.png");
        let keluaran = dir.join("out.png");
        // buat gambar 64x64
        let img = ImageBuffer::from_pixel(64, 64, Rgba([10u8, 20, 30, 255]));
        img.save(&masukan).unwrap();
        sisipkan_pesan_gambar(
            masukan.to_str().unwrap(),
            keluaran.to_str().unwrap(),
            "pesan rahasia",
            "sandi",
        )
        .unwrap();
        let hasil = baca_pesan_gambar(keluaran.to_str().unwrap(), "sandi").unwrap();
        assert_eq!(hasil, "pesan rahasia");
    }

    #[test]
    fn kapasitas_tidak_cukup() {
        let dir = temp_dir();
        let masukan = dir.join("small.png");
        let keluaran = dir.join("smallout.png");
        let img = ImageBuffer::from_pixel(2, 2, Rgba([10u8, 20, 30, 255]));
        img.save(&masukan).unwrap();
        let besar = "a".repeat(1000);
        let res = sisipkan_pesan_gambar(
            masukan.to_str().unwrap(),
            keluaran.to_str().unwrap(),
            &besar,
            "sandi",
        );
        assert!(res.is_err());
    }
}
