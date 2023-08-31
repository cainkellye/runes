//mod widgets;
mod wiregrid;
use std::{sync::{Arc, Mutex}, thread};

use runes_core::{game::{Game, Move, Player, PLAYER_SYMBOLS}, board::Position, ai_player::{AiPlayerMonte, Level}};
use wiregrid::WireGrid;

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
    pub birth: TextureId,
    pub gift: TextureId,
    pub wealth: TextureId,
    pub knowledge: TextureId,
    pub joy: TextureId,
    pub stone1: TextureId,
}

struct MyEguiApp {
    size: usize,
    store: Vec<RetainedImage>,
    pub images: TextureIds,
    game: Game,
    ai_level: Level,
    ai_move: Arc<Mutex<Option<Move>>>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let store = vec![
            RetainedImage::from_image_bytes(
            "birth.png", include_bytes!("../res/birth.png"),
            ).unwrap(),
            RetainedImage::from_image_bytes(
            "gift.png", include_bytes!("../res/gift.png"),
            ).unwrap(),
            RetainedImage::from_image_bytes(
            "knowledge.png", include_bytes!("../res/knowledge.png"),
            ).unwrap(),
            RetainedImage::from_image_bytes(
            "wealth.png", include_bytes!("../res/wealth.png"),
            ).unwrap(),
            RetainedImage::from_image_bytes(
            "joy.png", include_bytes!("../res/joy.png"),
            ).unwrap(),
            RetainedImage::from_image_bytes(
            "joy.png", include_bytes!("../res/stone1.png"),
            ).unwrap(),
        ];

        let ai_level = Level::Medium;
        let mut game = Game::new(13);
        let ai_first = AiPlayerMonte::new(Level::Medium).make_move(game.clone());
        game.apply_move(ai_first).unwrap();

        Self {
            size: game.board.size,
            images: TextureIds { 
                birth: store[0].texture_id(&cc.egui_ctx),
                gift: store[1].texture_id(&cc.egui_ctx),
                knowledge: store[2].texture_id(&cc.egui_ctx),
                wealth: store[3].texture_id(&cc.egui_ctx),
                joy: store[4].texture_id(&cc.egui_ctx),
                stone1: store[5].texture_id(&&cc.egui_ctx),
            },
            store,
            game,
            ai_level,
            ai_move: Arc::new(Mutex::new(None)),
        }
    }

    fn main_ui(&mut self, ui: &mut Ui) -> InnerResponse<()> {
        if let Some(ai_move) = self.ai_move.lock().unwrap().take() {
            self.game.apply_move(ai_move).unwrap();
        }
        
        ui.vertical(|ui| {
            if ui.button("New Game").clicked() {
                self.game = Game::new(self.game.board.size);
                let ai_first = AiPlayerMonte::new(Level::Medium).make_move(self.game.clone());
                self.game.apply_move(ai_first).unwrap();
            };
            let grid_response = ui
                .add(WireGrid {
                    board: self.game.board.clone(),
                    textures: self.images,
                });
            if !self.game.game_over && grid_response.clicked()
            {
                let clicked = WireGrid::get_clicked_cell(self.size, &grid_response);
                match self.game.apply_best_move_at(&Position(clicked.0, clicked.1)) {
                    Ok(_) if !self.game.game_over => {
                        let ai_move = self.ai_move.clone();
                        let ai_board = self.game.clone();
                        let ai_level = self.ai_level.clone();
                        thread::spawn(move || {
                            let ai = AiPlayerMonte::new(ai_level);
                            let made = ai.make_move(ai_board);
                            let mut x = ai_move.lock().unwrap();
                            *x = Some(made);
                        });
                    },
                    Ok(_) => (),
                    Err(_) => (), //Messagebox
                }
            };
        })
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.main_ui(ui));
    }
}
