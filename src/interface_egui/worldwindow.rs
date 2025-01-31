use egui::{
    epaint::{RectShape, Shape},
    Color32, Painter, Pos2, Rect, Rounding, Stroke, Vec2,
};

use crate::{
    datatypes::Coordinate,
    game_state::world::{FloorTile, WorldCell},
    interface::widgets::WorldWindowWidget,
};

impl WorldCell<'_> {
 pub   fn as_shape(&self, area: Rect) -> Vec<Shape> {
    let mut ret = Vec::new();
        let color = match self.floor {
            FloorTile::Water => Color32::DARK_BLUE,
            FloorTile::Stone => Color32::DARK_GRAY,
            FloorTile::Dirt => Color32::ORANGE,
        };

        ret.push(
        Shape::Rect(RectShape::new(
            area,
            Rounding::ZERO,
            color,
            Stroke::NONE,
        )));
        if let Some(actor) = self.actor {
            ret.push(
                Shape::Rect(RectShape::new(
                area,
                Rounding::ZERO,
                Color32::DARK_RED,
                Stroke::NONE,
            )));
        }
        ret
    }
}

impl WorldWindowWidget<'_> {
   pub fn paint(self, area: Rect) -> Vec<Shape> {
        let size = area.size();
        let cell_size = Vec2 { x: 16.0, y: 16.0 };

        let rows = (size.y / cell_size.y) as i32;
        let cols = (size.x / cell_size.x) as i32;

        let (centerx, centery) = (cols / 2 - self.center.x, rows / 2 + self.center.y);

        let mut ret = Vec::new();

        for i in 0..cols {
            for j in 0..rows {
                let x = i - centerx;
                let y = centery - j;
                let coord = Coordinate { x, y };
                let sub_area = Rect {
                    min: Pos2 {
                        x: i as f32 * cell_size.x,
                        y: j as f32 * cell_size.y,
                    },
                    max: Pos2 {
                        x: (i + 1) as f32 * cell_size.x,
                        y: (j + 1) as f32 * cell_size.y,
                    },
                };
                if self.world.actors.in_bounds(&coord) {
                    let cell = self
                        .world
                        .get_cell(&coord)
                        .expect("Cell out of bounds but was checked in bounds.");

                    ret.extend(cell.as_shape(sub_area));
                }
            }
        }
        ret
    }
}
