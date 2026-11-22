// Exploratory testing: berbagai input.
use aplikasi_steganografi::inti::utilitas;

#[test]
fn eksplorasi_header_besar() {
    let h = utilitas::buat_header(1000, 999);
    let (p, c) = utilitas::baca_header(&h).unwrap();
    assert_eq!(p, 1000);
    assert_eq!(c, 999);
}
