mod archive_viewer;
mod game;
mod util;

use std::sync::Arc;

use eframe::NativeOptions;
use egui::IconData;
use rand::RngExt;

use archive_viewer::viewer::ArchiveViewer;

static BONUS_ICONS: [&[u8]; 13] = [
    include_bytes!("../assets/bonus_icon_0.png"),
    include_bytes!("../assets/bonus_icon_1.png"),
    include_bytes!("../assets/bonus_icon_2.png"),
    include_bytes!("../assets/bonus_icon_3.png"),
    include_bytes!("../assets/bonus_icon_4.png"),
    include_bytes!("../assets/bonus_icon_5.png"),
    include_bytes!("../assets/bonus_icon_6.png"),
    include_bytes!("../assets/bonus_icon_7.png"),
    include_bytes!("../assets/bonus_icon_8.png"),
    include_bytes!("../assets/bonus_icon_9.png"),
    include_bytes!("../assets/bonus_icon_10.png"),
    include_bytes!("../assets/bonus_icon_11.png"),
    include_bytes!("../assets/bonus_icon_12.png"),
];

static ICON: &[u8] = include_bytes!("../assets/icon.png");

fn random_icon() -> &'static [u8] {
    // 1 in 10 chance
    let result = rand::rng().random_range(1..=10);

    if result == 10 {
        BONUS_ICONS[rand::rng().random_range(0..BONUS_ICONS.len())]
    } else {
        ICON
    }
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
                let icon = random_icon();
                let image = image::load_from_memory(icon)
                    .expect("Failed to open icon")
                    .into_rgba8();

                image.into_raw()
            },

            width: 64,
            height: 64,
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
