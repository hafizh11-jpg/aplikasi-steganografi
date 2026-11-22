// Sanity testing: fungsi dasar berjalan.
use aplikasi_steganografi::inti::utilitas;

#[test]
fn sanity_header() {
    let h = utilitas::buat_header(1, 2);
    let (p, c) = utilitas::baca_header(&h).unwrap();
    assert_eq!((p, c), (1, 2));
}
