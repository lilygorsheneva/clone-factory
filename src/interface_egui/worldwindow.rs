use egui::{
    epaint::{RectShape, Shape},
    pos2, Color32, Pos2, Rect, Rounding, Stroke, TextureOptions, Vec2,
};

use crate::{
    actor,
    datatypes::Coordinate,
    game_state::{
        game::Game,
        world::{FloorTile, World, WorldCell},
    },
    static_data::Data,
};

pub struct WorldWindowWidget<'a> {
    pub world: &'a World,
    pub center: Coordinate,
    data: &'a Data,
    pub show_cursor: bool,
}

impl<'a> WorldWindowWidget<'a> {
    pub fn new(game: &'a Game) -> WorldWindowWidget<'a> {
        WorldWindowWidget {
            world: &game.world,
            center: *game
                .get_player_coords()
                .unwrap_or(&Coordinate { x: 0, y: 0 }),
            data: &game.data,
            show_cursor: false,
        }
    }
}

impl WorldCell<'_> {
    pub fn as_shape(&self, ctx: &egui::Context, area: Rect) -> Vec<Shape> {
        let mut ret = Vec::new();
        let color = match self.floor {
            FloorTile::Water => Color32::DARK_BLUE,
            FloorTile::Stone => Color32::DARK_GRAY,
            FloorTile::Dirt => Color32::ORANGE,
        };

        ret.push(Shape::Rect(RectShape::new(
            area,
            Rounding::ZERO,
            color,
            Stroke::new(self.paradox.0 as f32 / 5.0 , Color32::WHITE),
        )));

        if let Some(building) = self.building {
            ret.push(Shape::Rect(RectShape::new(
                area.scale_from_center(0.9),
                Rounding::ZERO,
                Color32::WHITE,
                Stroke::NONE,
            )));
        }

        if let Some(item) = self.items[0] {
            ret.push(Shape::Rect(RectShape::new(
                area.scale_from_center(0.8),
                Rounding::ZERO,
                Color32::DARK_GREEN,
                Stroke::NONE,
            )));
        }



        if let Some(actor) = self.actor {
            let mut actor_sprite = RectShape::new(
                area.scale_from_center(0.7),
                Rounding::ZERO,
                Color32::DARK_RED,
                Stroke::NONE,
            );
            if let Some(path) = &actor.descriptor.appearance.texture {
                let tex = ctx.try_load_texture(
                    path,
                    TextureOptions::NEAREST,
                    egui::SizeHint::Scale(1.0.into()),
                );
         
        
                if let Ok(poll) = tex {
                    if let Some(id) = poll.texture_id() {
                        actor_sprite.fill = Color32::WHITE;
                        actor_sprite.fill_texture_id = id;
                        actor_sprite.uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
                    }
                }
            }


            ret.push(Shape::Rect(actor_sprite));
        }
        ret
    }
}

impl WorldWindowWidget<'_> {
    pub fn paint(self, ctx: &egui::Context, area: Rect) -> Vec<Shape> {
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

                    ret.extend(cell.as_shape(&ctx, sub_area));
                }
            }
        }
        ret
    }
}
