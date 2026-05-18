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
    fn new(special: bool) -> Self {
        Self {
            archive_viewer: ArchiveViewer::new(special),
        }
    }

    pub fn run() -> Result<(), eframe::Error> {
        let mut options = NativeOptions::default();

        let icon_data = random_icon();
        let special = icon_data != ICON;

        // load icon
        options.viewport.icon = Some(Arc::new({
            let image = image::load_from_memory(icon_data)
                .expect("Failed to open icon")
                .into_rgba8();

            let (width, height) = image.dimensions();

            IconData {
                rgba: image.into_raw(),
                width,
                height,
            }
        }));

        eframe::run_native(
            "Merino",
            options,
            Box::new(|cc| {
                setup_fonts(&cc.egui_ctx);

                Ok(Box::<MerinoApp>::from(MerinoApp::new(special)))
            }),
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

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "FiraCode".to_string(),
        egui::FontData::from_static(include_bytes!("../assets/font/FiraCode-Regular.ttf")).into(),
    );

    fonts.font_data.insert(
        "NotoSansJP".to_string(),
        egui::FontData::from_static(include_bytes!("../assets/font/NotoSansJP-Regular.ttf")).into(),
    );

    let monospace = fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default();

    monospace.insert(0, "FiraCode".to_string());
    monospace.push("NotoSansJP".to_string());

    ctx.set_fonts(fonts);
}
