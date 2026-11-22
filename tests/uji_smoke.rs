// Smoke testing: aplikasi dapat dijalankan dan modul inti tersedia.
use aplikasi_steganografi::inti::utilitas;

#[test]
fn smoke_utilitas_tersedia() {
    assert!(!utilitas::MAGIC_STEGO.is_empty());
}
