// Jendela utama aplikasi steganografi.

use crate::inti::stego_audio;
use crate::inti::stego_gambar;
use crate::inti::stego_video;
use crate::inti::utilitas;
use eframe::egui;

/// Mode operasi aplikasi.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ModeOperasi {
    Sisipkan,
    Baca,
}

/// Jenis media target.
#[derive(Clone, Copy, PartialEq, Eq)]
enum JenisMedia {
    Gambar,
    Audio,
    Video,
}

/// Struktur state aplikasi.
pub struct JendelaUtama {
    mode: ModeOperasi,
    media: JenisMedia,
    jalur_masukan: String,
    jalur_keluaran: String,
    pesan: String,
    kata_sandi: String,
    status: String,
    sukses: bool,
}

impl Default for JendelaUtama {
    fn default() -> Self {
        Self {
            mode: ModeOperasi::Sisipkan,
            media: JenisMedia::Gambar,
            jalur_masukan: String::new(),
            jalur_keluaran: String::new(),
            pesan: String::new(),
            kata_sandi: String::new(),
            status: String::new(),
            sukses: false,
        }
    }
}

impl eframe::App for JendelaUtama {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::antarmuka::tema::terapkan_tema(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Aplikasi Steganografi");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Mode:");
                ui.selectable_value(&mut self.mode, ModeOperasi::Sisipkan, "Sisipkan");
                ui.selectable_value(&mut self.mode, ModeOperasi::Baca, "Baca");
            });

            ui.horizontal(|ui| {
                ui.label("Media:");
                ui.selectable_value(&mut self.media, JenisMedia::Gambar, "Gambar");
                ui.selectable_value(&mut self.media, JenisMedia::Audio, "Audio");
                ui.selectable_value(&mut self.media, JenisMedia::Video, "Video");
            });

            ui.separator();

            // Berkas masukan
            ui.horizontal(|ui| {
                ui.label("Berkas masukan:");
                ui.text_edit_singleline(&mut self.jalur_masukan);
                if ui.button("Pilih").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.jalur_masukan = path.display().to_string();
                    }
                }
            });

            // Berkas keluaran (hanya sisipkan)
            if self.mode == ModeOperasi::Sisipkan {
                ui.horizontal(|ui| {
                    ui.label("Berkas keluaran:");
                    ui.text_edit_singleline(&mut self.jalur_keluaran);
                    if ui.button("Simpan").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            self.jalur_keluaran = path.display().to_string();
                        }
                    }
                });
            }

            // Pesan
            if self.mode == ModeOperasi::Sisipkan {
                ui.label("Pesan rahasia:");
                ui.text_edit_multiline(&mut self.pesan);
            }

            // Kata sandi
            ui.horizontal(|ui| {
                ui.label("Kata sandi:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.kata_sandi).password(true),
                );
            });

            ui.separator();

            if ui.button("Jalankan").clicked() {
                self.jalankan();
            }

            ui.separator();
            crate::antarmuka::komponen::tampilkan_status(ui, &self.status, self.sukses);
        });
    }
}

impl JendelaUtama {
    fn jalankan(&mut self) {
        self.status.clear();
        self.sukses = false;

        match self.mode {
            ModeOperasi::Sisipkan => {
                if self.jalur_masukan.is_empty() || self.jalur_keluaran.is_empty() {
                    self.status = "Pilih berkas masukan dan keluaran.".into();
                    return;
                }
                let hasil = match self.media {
                    JenisMedia::Gambar => stego_gambar::sisipkan_pesan_gambar(
                        &self.jalur_masukan,
                        &self.jalur_keluaran,
                        &self.pesan,
                        &self.kata_sandi,
                    ),
                    JenisMedia::Audio => stego_audio::sisipkan_pesan_audio(
                        &self.jalur_masukan,
                        &self.jalur_keluaran,
                        &self.pesan,
                        &self.kata_sandi,
                    ),
                    JenisMedia::Video => stego_video::sisipkan_pesan_video(
                        &self.jalur_masukan,
                        &self.jalur_keluaran,
                        &self.pesan,
                        &self.kata_sandi,
                    ),
                };
                match hasil {
                    Ok(()) => {
                        self.status = "Pesan berhasil disisipkan.".into();
                        self.sukses = true;
                    }
                    Err(e) => {
                        self.status = format!("Gagal: {}", e);
                    }
                }
            }
            ModeOperasi::Baca => {
                if self.jalur_masukan.is_empty() {
                    self.status = "Pilih berkas masukan.".into();
                    return;
                }
                let hasil = match self.media {
                    JenisMedia::Gambar => {
                        stego_gambar::baca_pesan_gambar(&self.jalur_masukan, &self.kata_sandi)
                    }
                    JenisMedia::Audio => {
                        stego_audio::baca_pesan_audio(&self.jalur_masukan, &self.kata_sandi)
                    }
                    JenisMedia::Video => {
                        stego_video::baca_pesan_video(&self.jalur_masukan, &self.kata_sandi)
                    }
                };
                match hasil {
                    Ok(p) => {
                        self.pesan = p;
                        self.status = "Pesan berhasil dibaca.".into();
                        self.sukses = true;
                    }
                    Err(e) => {
                        self.status = format!("Gagal: {}", e);
                    }
                }
            }
        }
    }
}

/// Menjalankan aplikasi desktop.
pub fn jalankan_aplikasi() -> eframe::Result<()> {
    let opsi = eframe::NativeOptions::default();
    eframe::run_native(
        "Aplikasi Steganografi",
        opsi,
        Box::new(|_cc| Ok(Box::new(JendelaUtama::default()))),
    )
}
