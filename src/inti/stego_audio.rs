// Steganografi pada audio WAV (LSB pada sampel).

use crate::inti::kriptografi;
use crate::inti::utilitas;
use crate::kesalahan::KesalahanAplikasi;
use hound::{WavReader, WavSpec, WavWriter};

/// Menyisipkan pesan terenkripsi ke dalam berkas WAV.
pub fn sisipkan_pesan_audio(
    jalur_masukan: &str,
    jalur_keluaran: &str,
    pesan: &str,
    kata_sandi: &str,
) -> Result<(), KesalahanAplikasi> {
    utilitas::validasi_pesan(pesan)?;
    utilitas::validasi_kata_sandi(kata_sandi)?;

    let mut pembaca = WavReader::open(jalur_masukan)
        .map_err(|e| KesalahanAplikasi::GagalProsesAudio(e.to_string()))?;
    let spesifikasi = pembaca.spec();
    if spesifikasi.bits_per_sample != 16 {
        return Err(KesalahanAplikasi::FormatTidakDidukung(
            "hanya WAV 16-bit yang didukung".into(),
        ));
    }

    let sampel: Vec<i16> = pembaca
        .samples::<i16>()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| KesalahanAplikasi::GagalProsesAudio(e.to_string()))?;

    let data_terenkripsi = kriptografi::enkripsi(pesan.as_bytes(), kata_sandi)?;
    let checksum = utilitas::hitung_checksum(&data_terenkripsi);
    let mut payload = utilitas::buat_header(data_terenkripsi.len(), checksum);
    payload.extend_from_slice(&data_terenkripsi);

    let bit = utilitas::byte_ke_bit(&payload);
    if bit.len() > sampel.len() {
        return Err(KesalahanAplikasi::KapasitasTidakCukup);
    }

    let mut sampel_baru = sampel.clone();
    for (i, b) in bit.iter().enumerate() {
        let s = sampel_baru[i];
        sampel_baru[i] = (s & !1) | i16::from(*b);
    }

    let mut penulis = WavWriter::create(jalur_keluaran, spesifikasi)
        .map_err(|e| KesalahanAplikasi::GagalMenulisBerkas(e.to_string()))?;
    for s in sampel_baru {
        penulis
            .write_sample(s)
            .map_err(|e| KesalahanAplikasi::GagalMenulisBerkas(e.to_string()))?;
    }
    penulis
        .finalize()
        .map_err(|e| KesalahanAplikasi::GagalMenulisBerkas(e.to_string()))?;
    Ok(())
}

/// Membaca pesan tersembunyi dari berkas WAV.
pub fn baca_pesan_audio(
    jalur_masukan: &str,
    kata_sandi: &str,
) -> Result<String, KesalahanAplikasi> {
    utilitas::validasi_kata_sandi(kata_sandi)?;
    let mut pembaca = WavReader::open(jalur_masukan)
        .map_err(|e| KesalahanAplikasi::GagalProsesAudio(e.to_string()))?;
    let sampel: Vec<i16> = pembaca
        .samples::<i16>()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| KesalahanAplikasi::GagalProsesAudio(e.to_string()))?;

    if sampel.len() < 96 {
        return Err(KesalahanAplikasi::DataTidakDitemukan);
    }
    let bit: Vec<bool> = sampel.iter().map(|s| s & 1 == 1).collect();
    let header_byte = utilitas::bit_ke_byte(&bit[..96]);
    let (panjang, checksum) = utilitas::baca_header(&header_byte)?;

    let total_bit = (12 + panjang) * 8;
    if bit.len() < total_bit {
        return Err(KesalahanAplikasi::DataTidakDitemukan);
    }
    let data_terenkripsi = utilitas::bit_ke_byte(&bit[96..total_bit]);
    if utilitas::hitung_checksum(&data_terenkripsi) != checksum {
        return Err(KesalahanAplikasi::ChecksumTidakCocok);
    }
    let plaintext = kriptografi::dekripsi(&data_terenkripsi, kata_sandi)?;
    String::from_utf8(plaintext).map_err(|_| KesalahanAplikasi::DekripsiGagal)
}

#[cfg(test)]
mod uji {
    use super::*;
    use hound::{WavSpec, WavWriter};
    use std::env::temp_dir;

    fn buat_wav(path: &std::path::Path) {
        let spec = WavSpec {
            channels: 1,
            sample_rate: 8000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut w = WavWriter::create(path, spec).unwrap();
        for i in 0..4096i16 {
            w.write_sample(i).unwrap();
        }
        w.finalize().unwrap();
    }

    #[test]
    fn roundtrip_audio() {
        let dir = temp_dir();
        let masukan = dir.join("in.wav");
        let keluaran = dir.join("out.wav");
        buat_wav(&masukan);
        sisipkan_pesan_audio(
            masukan.to_str().unwrap(),
            keluaran.to_str().unwrap(),
            "audio rahasia",
            "sandi",
        )
        .unwrap();
        let hasil = baca_pesan_audio(keluaran.to_str().unwrap(), "sandi").unwrap();
        assert_eq!(hasil, "audio rahasia");
    }
}
