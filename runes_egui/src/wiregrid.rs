use eframe::egui;
use egui::{Color32, Pos2, Rect, Rounding, Sense, Stroke, Widget};
use runes_core::board::{Field, Position};

use crate::TextureIds;

pub struct WireGrid {
    pub textures: TextureIds,
    pub board: runes_core::board::Board,
}

impl WireGrid {
    pub fn calculate_cell_size(columns: usize, widget_bounds: Rect) -> f32 {
        (widget_bounds.height() / columns as f32).floor().max(1.0)
    }

    pub fn get_clicked_cell(size: usize, grid_response: &egui::Response) -> (usize, usize) {
        let cell_render_size = WireGrid::calculate_cell_size(size, grid_response.rect);
        let pos = grid_response.interact_pointer_pos().unwrap() - grid_response.rect.left_top();
        (
            ((pos.x / cell_render_size).floor() as usize).min(size - 1),
            ((pos.y / cell_render_size).floor() as usize).min(size - 1),
        )
    }
}

impl Widget for WireGrid {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_at_least(ui.available_size(), Sense::click());
        let cell_size = WireGrid::calculate_cell_size(self.board.size, rect);

        let coords =
            (0..self.board.size).flat_map(move |i| (0..self.board.size).map(move |j| (i, j)));

        coords.for_each(|(i, j)| {
            let field = self.board.field_at(&Position(i, j));

            let p1 = Pos2::new(
                i as f32 * cell_size + rect.left(),
                j as f32 * cell_size + rect.top(),
            );
            let p2 = Pos2::new(
                (i + 1) as f32 * cell_size + rect.left(),
                (j + 1) as f32 * cell_size + rect.top(),
            );

            match field {
                Field::Empty => ui.painter().rect(
                                    Rect::from_two_pos(p1, p2),
                                    Rounding::none(),
                                    Color32::from_rgb(80, 80, 80),
                                    Stroke::new(1.0, Color32::BLACK),
                                ),
                f => {
                    let uv = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0));
                    ui.painter().image(
                        match f {
                            Field::Birth => self.textures.birth,
                            Field::Gift => self.textures.gift,
                            Field::Wealth => self.textures.wealth,
                            Field::Knowledge => self.textures.knowledge,
                            Field::Joy => self.textures.joy,
                            Field::Empty => self.textures.birth,
                        },
                        Rect::from_min_max(p1, p2).shrink(3.0),
                        uv,
                        Color32::WHITE,
                    );
                },
            }
        });
        response
    }
}
