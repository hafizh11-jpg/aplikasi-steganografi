// Performance testing: operasi kriptografi selesai dalam waktu wajar.
use aplikasi_steganografi::inti::kriptografi;
use std::time::Instant;

#[test]
fn performa_enkripsi() {
    let data = vec![0u8; 1024 * 100];
    let mulai = Instant::now();
    let ct = kriptografi::enkripsi(&data, "sandi").unwrap();
    let _ = kriptografi::dekripsi(&ct, "sandi").unwrap();
    assert!(mulai.elapsed().as_secs() < 5);
}
