// Load/Stress testing: beban data besar.
use aplikasi_steganografi::inti::kriptografi;

#[test]
fn beban_data_besar() {
    let data = vec![7u8; 1024 * 1024];
    let ct = kriptografi::enkripsi(&data, "sandi").unwrap();
    let pt = kriptografi::dekripsi(&ct, "sandi").unwrap();
    assert_eq!(pt.len(), data.len());
}
