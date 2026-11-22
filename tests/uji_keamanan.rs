// Security testing: verifikasi keamanan dasar.
use aplikasi_steganografi::inti::kriptografi;

#[test]
fn keamanan_sandi_salah() {
    let ct = kriptografi::enkripsi(b"rahasia", "benar").unwrap();
    assert!(kriptografi::dekripsi(&ct, "salah").is_err());
}

#[test]
fn keamanan_data_rusak() {
    let mut ct = kriptografi::enkripsi(b"rahasia", "benar").unwrap();
    if let Some(b) = ct.last_mut() {
        *b ^= 0xFF;
    }
    assert!(kriptografi::dekripsi(&ct, "benar").is_err());
}
