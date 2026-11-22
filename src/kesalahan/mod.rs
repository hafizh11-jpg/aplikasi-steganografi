// Modul kesalahan aplikasi.
// Berisi definisi tipe kesalahan yang digunakan di seluruh aplikasi.

/// Tipe kesalahan utama aplikasi steganografi.
#[derive(Debug, thiserror::Error)]
pub enum KesalahanAplikasi {
    #[error("Berkas tidak ditemukan: {0}")]
    BerkasTidakDitemukan(String),

    #[error("Format berkas tidak didukung: {0}")]
    FormatTidakDidukung(String),

    #[error("Kapasitas media tidak cukup untuk menyimpan pesan")]
    KapasitasTidakCukup,

    #[error("Kata sandi tidak boleh kosong")]
    KataSandiKosong,

    #[error("Pesan tidak boleh kosong")]
    PesanKosong,

    #[error("Gagal membaca berkas: {0}")]
    GagalMembacaBerkas(String),

    #[error("Gagal menulis berkas: {0}")]
    GagalMenulisBerkas(String),

    #[error("Gagal memproses gambar: {0}")]
    GagalProsesGambar(String),

    #[error("Gagal memproses audio: {0}")]
    GagalProsesAudio(String),

    #[error("Gagal memproses video: {0}")]
    GagalProsesVideo(String),

    #[error("Dekripsi gagal: kata sandi salah atau data rusak")]
    DekripsiGagal,

    #[error("Data steganografi tidak ditemukan pada media")]
    DataTidakDitemukan,

    #[error("Checksum tidak cocok: data mungkin rusak")]
    ChecksumTidakCocok,

    #[error("Kesalahan kriptografi: {0}")]
    Kriptografi(String),

    #[error("Kesalahan I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("Kesalahan gambar: {0}")]
    Gambar(#[from] image::ImageError),
}
