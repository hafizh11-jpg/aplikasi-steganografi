// Steganografi pada video (LSB pada frame pertama).

use crate::inti::stego_gambar;
use crate::inti::utilitas;
use crate::kesalahan::KesalahanAplikasi;
use std::fs;
use std::process::Command;

/// Membuat direktori sementara unik.
fn direktori_sementara() -> Result<std::path::PathBuf, KesalahanAplikasi> {
    let mut path = std::env::temp_dir();
    path.push(format!("stego_video_{}", std::process::id()));
    fs::create_dir_all(&path)
        .map_err(|e| KesalahanAplikasi::GagalProsesVideo(e.to_string()))?;
    Ok(path)
}

/// Menjalankan perintah eksternal.
fn jalankan_perintah(program: &str, args: &[&str]) -> Result<(), KesalahanAplikasi> {
    let status = Command::new(program)
        .args(args)
        .status()
        .map_err(|e| KesalahanAplikasi::GagalProsesVideo(e.to_string()))?;
    if !status.success() {
        return Err(KesalahanAplikasi::GagalProsesVideo(
            format!("perintah {} gagal", program),
        ));
    }
    Ok(())
}

/// Mengambil frame rate video.
fn ambil_fps(jalur: &str) -> Result<String, KesalahanAplikasi> {
    let out = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=r_frame_rate",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            jalur,
        ])
        .output()
        .map_err(|e| KesalahanAplikasi::GagalProsesVideo(e.to_string()))?;
    let fps = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if fps.is_empty() {
        return Ok("30".to_string());
    }
    Ok(fps)
}

/// Menyisipkan pesan ke dalam video (frame pertama).
pub fn sisipkan_pesan_video(
    jalur_masukan: &str,
    jalur_keluaran: &str,
    pesan: &str,
    kata_sandi: &str,
) -> Result<(), KesalahanAplikasi> {
    utilitas::validasi_pesan(pesan)?;
    utilitas::validasi_kata_sandi(kata_sandi)?;

    let dir = direktori_sementara()?;
    let pola_frame = dir.join("frame_%04d.png");
    let pola_str = pola_frame.to_string_lossy().to_string();

    // Ekstrak frame
    jalankan_perintah(
        "ffmpeg",
        &["-y", "-i", jalur_masukan, &pola_str],
    )?;

    // Sisipkan pesan pada frame pertama
    let frame_pertama = dir.join("frame_0001.png");
    if !frame_pertama.exists() {
        let _ = fs::remove_dir_all(&dir);
        return Err(KesalahanAplikasi::GagalProsesVideo(
            "frame tidak ditemukan".into(),
        ));
    }
    let fp = frame_pertama.to_string_lossy().to_string();
    stego_gambar::sisipkan_pesan_gambar(&fp, &fp, pesan, kata_sandi)?;

    let fps = ambil_fps(jalur_masukan)?;
    let keluaran = jalur_keluaran.to_string();

    // Rakit ulang video dengan audio asli
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-framerate",
            &fps,
            "-i",
            &pola_str,
            "-i",
            jalur_masukan,
            "-map",
            "0:v",
            "-map",
            "1:a?",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "copy",
            &keluaran,
        ])
        .status()
        .map_err(|e| KesalahanAplikasi::GagalProsesVideo(e.to_string()))?;

    let _ = fs::remove_dir_all(&dir);
    if !status.success() {
        return Err(KesalahanAplikasi::GagalProsesVideo(
            "perakitan video gagal".into(),
        ));
    }
    Ok(())
}

/// Membaca pesan tersembunyi dari video (frame pertama).
pub fn baca_pesan_video(
    jalur_masukan: &str,
    kata_sandi: &str,
) -> Result<String, KesalahanAplikasi> {
    utilitas::validasi_kata_sandi(kata_sandi)?;
    let dir = direktori_sementara()?;
    let pola_frame = dir.join("frame_%04d.png");
    let pola_str = pola_frame.to_string_lossy().to_string();

    jalankan_perintah("ffmpeg", &["-y", "-i", jalur_masukan, "-frames:v", "1", &pola_str])?;

    let frame_pertama = dir.join("frame_0001.png");
    if !frame_pertama.exists() {
        let _ = fs::remove_dir_all(&dir);
        return Err(KesalahanAplikasi::GagalProsesVideo(
            "frame tidak ditemukan".into(),
        ));
    }
    let fp = frame_pertama.to_string_lossy().to_string();
    let hasil = stego_gambar::baca_pesan_gambar(&fp, kata_sandi);
    let _ = fs::remove_dir_all(&dir);
    hasil
}
