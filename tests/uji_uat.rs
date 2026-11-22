// UAT: skenario pengguna menyisipkan dan membaca pesan.
use aplikasi_steganografi::inti::stego_gambar;
use image::{ImageBuffer, Rgba};
use std::env::temp_dir;

#[test]
fn uat_skenario_gambar() {
    let dir = temp_dir();
    let m = dir.join("uat_in.png");
    let k = dir.join("uat_out.png");
    ImageBuffer::from_pixel(32, 32, Rgba([1u8, 2, 3, 255]))
        .save(&m)
        .unwrap();
    stego_gambar::sisipkan_pesan_gambar(
        m.to_str().unwrap(),
        k.to_str().unwrap(),
        "uat pesan",
        "uatpw",
    )
    .unwrap();
    let p = stego_gambar::baca_pesan_gambar(k.to_str().unwrap(), "uatpw").unwrap();
    assert_eq!(p, "uat pesan");
}
