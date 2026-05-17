mod archive_viewer;
mod game;
mod util;

use std::sync::Arc;

use eframe::NativeOptions;
use egui::IconData;
use rand::RngExt;

use archive_viewer::viewer::ArchiveViewer;

static ICONS: [&[u8]; 13] = [
    include_bytes!("../assets/icon_0.png"),
    include_bytes!("../assets/icon_1.png"),
    include_bytes!("../assets/icon_2.png"),
    include_bytes!("../assets/icon_3.png"),
    include_bytes!("../assets/icon_4.png"),
    include_bytes!("../assets/icon_5.png"),
    include_bytes!("../assets/icon_6.png"),
    include_bytes!("../assets/icon_7.png"),
    include_bytes!("../assets/icon_8.png"),
    include_bytes!("../assets/icon_9.png"),
    include_bytes!("../assets/icon_10.png"),
    include_bytes!("../assets/icon_11.png"),
    include_bytes!("../assets/icon_12.png"),
];

fn random_icon_index() -> usize {
    rand::rng().random_range(0..ICONS.len())
}

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
        let mut options = NativeOptions::default();

        // load icon
        options.viewport.icon = Some(Arc::new(IconData {
            rgba: {
                let icon = ICONS[random_icon_index()];
                let image = image::load_from_memory(icon).expect("Failed to open icon").into_rgba8();

                image.into_raw()
            },

            width: 64,
            height: 64
        }));


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

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.archive_viewer.on_exit();
    }
}
