//mod widgets;
mod wiregrid;
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
}

struct MyEguiApp {
    size: usize,
    store: Vec<RetainedImage>,
    pub images: TextureIds,
    game: Game,
    ai_player: Box<dyn Player>,
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
        ];

        let mut ai_player = Box::new(AiPlayerMonte::new(Level::Medium));
        //let player1 = Box::new(AiPlayer::new(Level::Medium));
        //let player2 = Box::new(AiPlayerMonte::new(Level::Easy));
        let mut game = Game::new(13);
        ai_player.set_symbol(PLAYER_SYMBOLS[0]);
        let ai_first = ai_player.make_move(game.clone());
        game.apply_move(ai_first).unwrap();

        Self {
            size: game.board.size,
            images: TextureIds { 
                birth: store[0].texture_id(&cc.egui_ctx),
                gift: store[1].texture_id(&cc.egui_ctx),
                knowledge: store[2].texture_id(&cc.egui_ctx),
                wealth: store[3].texture_id(&cc.egui_ctx),
                joy: store[4].texture_id(&cc.egui_ctx),
            },
            store,
            game,
            ai_player,
        }
    }

    fn main_ui(&mut self, ui: &mut Ui) -> InnerResponse<()> {
        ui.vertical(|ui| {
            if ui.button("New Game").clicked() {
                self.game = Game::new(self.game.board.size);
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
                    Ok(_) => if !self.game.game_over {
                        let ai_move = self.ai_player.make_move(self.game.clone());
                        self.game.apply_move(ai_move).unwrap();
                    },
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
