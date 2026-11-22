// Komponen UI yang dapat digunakan ulang.

use eframe::egui;

/// Menampilkan label status dengan warna sesuai jenis.
pub fn tampilkan_status(ui: &mut egui::Ui, pesan: &str, sukses: bool) {
    if pesan.is_empty() {
        return;
    }
    let warna = if sukses {
        egui::Color32::from_rgb(0, 128, 0)
    } else {
        egui::Color32::from_rgb(180, 0, 0)
    };
    ui.colored_label(warna, pesan);
}

/// Tombol pilih berkas dengan label.
pub fn tombol_pilih(ui: &mut egui::Ui, label: &str) -> bool {
    ui.button(label).clicked()
}
