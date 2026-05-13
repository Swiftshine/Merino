mod archive_viewer;
mod game;
mod util;

use eframe::NativeOptions;

use archive_viewer::viewer::ArchiveViewer;

/// Contains app data.
pub struct MerinoApp {
    archive_viewer: ArchiveViewer,
}

impl MerinoApp {
    fn new() -> Self {
        Self {
            archive_viewer: ArchiveViewer::new(),
        }
    }

    pub fn run() -> Result<(), eframe::Error> {
        let options = NativeOptions::default();

        eframe::run_native(
            "Merino",
            options,
            Box::new(|_cc| Ok(Box::<MerinoApp>::from(MerinoApp::new()))),
        )
    }
}

impl eframe::App for MerinoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.archive_viewer.show_ui(ui);
        });
    }
}
