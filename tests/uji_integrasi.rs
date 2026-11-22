// Pengujian integrasi aplikasi steganografi.

use aplikasi_steganografi::inti::stego_gambar;
use aplikasi_steganografi::inti::utilitas;
use image::{ImageBuffer, Rgba};
use std::env::temp_dir;

#[test]
fn integrasi_sisip_baca_gambar() {
    let dir = temp_dir();
    let masukan = dir.join("int_in.png");
    let keluaran = dir.join("int_out.png");
    let img = ImageBuffer::from_pixel(128, 128, Rgba([5u8, 10, 15, 255]));
    img.save(&masukan).unwrap();
    stego_gambar::sisipkan_pesan_gambar(
        masukan.to_str().unwrap(),
        keluaran.to_str().unwrap(),
        "integrasi ok",
        "rahasia",
    )
    .unwrap();
    let hasil = stego_gambar::baca_pesan_gambar(keluaran.to_str().unwrap(), "rahasia").unwrap();
    assert_eq!(hasil, "integrasi ok");
}

#[test]
fn integrasi_checksum() {
    let data = b"integrasi";
    let c = utilitas::hitung_checksum(data);
    assert_eq!(utilitas::hitung_checksum(data), c);
}
