mod widgets;
use widgets::*;

use eframe::egui;
use egui::{InnerResponse, Slider, TextureId, Ui, Vec2};
use egui_extras::RetainedImage;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "MyApp",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

#[derive(Clone, Copy)]
pub struct TextureIds {
    pub sunglass: TextureId,
}

struct MyEguiApp {
    size: i32,
    cell_size: i32,
    #[allow(dead_code)]
    store: Vec<RetainedImage>,
    pub images: TextureIds,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let store = vec![RetainedImage::from_image_bytes(
            "sunglass.png",
            include_bytes!("../res/sunglass.png"),
        )
        .unwrap()];
        Self {
            size: 13,
            cell_size: 40,
            images: TextureIds { sunglass: store[0].texture_id(&cc.egui_ctx) },
            store,
        }
    }

    fn main_ui(&mut self, ui: &mut Ui) -> InnerResponse<()> {
        ui.vertical(|ui| {
            ui.add(Slider::new(&mut self.cell_size, 10..=50));
            if ui
                .add(WireGrid {
                    rows: self.size,
                    columns: self.size,
                    cell_size: Vec2::splat(self.cell_size as f32),
                    textures: self.images,
                })
                .clicked()
            {
                self.cell_size *= 2;
            }
        })
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.main_ui(ui));
    }
}
