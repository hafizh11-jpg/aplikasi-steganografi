// Titik masuk aplikasi steganografi.

use aplikasi_steganografi::antarmuka::jendela_utama::jalankan_aplikasi;

fn main() {
    if let Err(e) = jalankan_aplikasi() {
        eprintln!("Gagal menjalankan aplikasi: {}", e);
    }
}
