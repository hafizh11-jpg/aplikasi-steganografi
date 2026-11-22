// Regression testing: roundtrip tetap berfungsi setelah perubahan.
use aplikasi_steganografi::inti::kriptografi;

#[test]
fn regresi_kripto() {
    let ct = kriptografi::enkripsi(b"regresi", "pw").unwrap();
    let pt = kriptografi::dekripsi(&ct, "pw").unwrap();
    assert_eq!(pt, b"regresi");
}
