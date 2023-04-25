use egui::{Widget, Vec2, Sense, Rect, Pos2, Stroke, Color32};

use crate::TextureIds;


pub struct WireGrid {
    pub rows: i32,
    pub columns: i32,
    pub cell_size: Vec2,
    pub textures: TextureIds,
}

impl Widget for WireGrid {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = Vec2::new(self.cell_size.x * self.columns as f32, self.cell_size.y * self.rows as f32);
        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());

        let painter = ui.painter_at(rect);
        // let mut sungl = egui::Mesh::with_texture(self.images.sunglass.texture_id(ui.ctx()));
        // sungl.add_rect_with_uv(
        //     Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(40.0, 40.0)), 
        //     Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
        //     Color32::from_white_alpha(0));
        // sungl.translate(Vec2::splat(50.0));
        //println!("{:?}", sungl.calc_bounds());
        //painter.add(sungl);
        // painter.image(self.images.sunglass.texture_id(ui.ctx()), 
        //     Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(40.0, 40.0)), 
        //     Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
        //     Color32::from_white_alpha(255));
        let uv = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0));
        for i in 0..self.columns {
            for j in 0..self.rows {
                let p1 = Pos2::new(i as f32 * self.cell_size.x + rect.left(), j as f32 * self.cell_size.y + rect.top());
                let p2 = Pos2::new((i + 1) as f32 * self.cell_size.x + rect.left(), (j + 1) as f32 * self.cell_size.y + rect.top());
                painter.image(self.textures.sunglass, 
                    Rect::from_min_max(p1, p2).shrink(3.0), 
                    uv,
                    Color32::WHITE);
                painter.add(egui::epaint::RectShape::stroke(
                    Rect::from_two_pos(p1, p2),
                    0.0,
                    Stroke::new(1.0, Color32::BLACK),
                ));
            }
        }

        response
    }
}