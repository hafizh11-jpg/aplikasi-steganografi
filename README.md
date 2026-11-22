# Aplikasi Steganografi

Aplikasi desktop offline untuk menyisipkan dan membaca pesan tersembunyi pada berbagai media
(gambar, audio, dan video). Dibangun menggunakan bahasa pemrograman Rust dengan antarmuka grafis.

## Fitur Utama

- Steganografi pada gambar (PNG/BMP) menggunakan metode LSB pada kanal RGB.
- Steganografi pada audio (WAV 16-bit) menggunakan metode LSB pada sampel.
- Steganografi pada video (frame pertama) menggunakan metode LSB.
- Enkripsi pesan menggunakan ChaCha20-Poly1305 dengan derivasi kunci Argon2id.
- Kompresi tidak wajib; data diverifikasi dengan checksum CRC32.
- Antarmuka grafis sederhana dengan dukungan pemilih berkas.
- Validasi input dan penanganan kesalahan yang komprehensif.

## Keamanan

- Penggunaan AEAD (ChaCha20-Poly1305) untuk kerahasiaan dan integritas.
- Derivasi kunci dari kata sandi menggunakan Argon2id.
- Pembersihan memori kunci menggunakan zeroize.
- Profil rilis dioptimalkan dengan LTO, strip simbol, dan panic abort untuk
  mempersulit reverse engineering.
- Validasi kapasitas media dan batas ukuran pesan.

## Struktur Proyek

```
src/
  kesalahan/       Modul kesalahan aplikasi
  inti/            Modul inti (kriptografi, steganografi)
  antarmuka/       Modul antarmuka pengguna
tests/             Pengujian integrasi dan lainnya
biner/             Berkas biner rilis
```

## Persyaratan

- Rust (edisi 2024) dan Cargo.
- Pustaka sistem: libx11, wayland (opsional), pkg-config.
- ffmpeg dan ffprobe untuk fitur video.

## Cara Membangun

```bash
export PKG_CONFIG_PATH=/home/linuxbrew/.linuxbrew/lib/pkgconfig
cargo build --release
```

Biner hasil berada di `target/release/aplikasi-steganografi`.

Untuk Windows:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

## Cara Menjalankan

```bash
./target/release/aplikasi-steganografi
```

## Pengujian

```bash
cargo test
```

Pengujian mencakup unit, integrasi, smoke, sanity, regresi, keamanan, performa,
beban, UAT, eksplorasi, dan mocking.

## Catatan

- Hanya gunakan pada media milik sendiri atau dengan izin.
- Video diproses melalui frame pertama; kualitas dapat berubah karena re-encoding.
