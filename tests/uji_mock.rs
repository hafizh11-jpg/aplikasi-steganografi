// Mocking: simulasi objek media untuk pengujian.
struct MediaPalsu {
    data: Vec<u8>,
}

impl MediaPalsu {
    fn baca(&self) -> &[u8] {
        &self.data
    }
}

#[test]
fn mock_media() {
    let m = MediaPalsu { data: vec![1, 2, 3] };
    assert_eq!(m.baca(), &[1, 2, 3]);
}
