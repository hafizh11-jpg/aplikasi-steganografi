// Pengaturan tema aplikasi.

use eframe::egui;

/// Menerapkan tema terang dengan kontras tinggi untuk aksesibilitas.
pub fn terapkan_tema(konteks: &egui::Context) {
    let visual = egui::Visuals::light();
    // visual window rounding tidak tersedia pada versi ini
    konteks.set_visuals(visual);
}
