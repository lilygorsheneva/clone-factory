use std::f32;

use egui::{
    emath::Rot2, epaint::{CircleShape, RectShape, Shape}, pos2, Color32, Mesh, Pos2, Rect, Rounding, Stroke, TextureOptions, Vec2
};

use crate::{
    datatypes::Coordinate,
    direction::AbsoluteDirection,
    game_state::{
        game::Game,
        world::{FloorTile, World, WorldCell},
    },
    static_data::AppearanceDefiniton,
};

pub struct WorldWindowWidget<'a> {
    pub world: &'a World,
    pub center: Coordinate,
    pub show_cursor: bool,
}

impl<'a> WorldWindowWidget<'a> {
    pub fn new(game: &'a Game) -> WorldWindowWidget<'a> {
        WorldWindowWidget {
            world: &game.world,
            center: *game
                .get_player_coords()
                .unwrap_or(&Coordinate { x: 0, y: 0 }),
            show_cursor: false,
        }
    }
}

fn object_to_shape(
    ctx: &egui::Context,
    descriptor: &AppearanceDefiniton,
    area: Rect,
    scale: f32,
    orientation: AbsoluteDirection,
) -> Shape {
    let rot = match orientation {
        AbsoluteDirection::N => Rot2::from_angle(f32::consts::PI),
        AbsoluteDirection::E => Rot2::from_angle(f32::consts::FRAC_PI_2*3.0),
        AbsoluteDirection::S => Rot2::from_angle(0.0),
        AbsoluteDirection::W => Rot2::from_angle(f32::consts::FRAC_PI_2),
    };
    if let Some(path) = &descriptor.texture {
        let tex = ctx.try_load_texture(
            path,
            TextureOptions::NEAREST,
            egui::SizeHint::Scale(1.0.into()),
        );

        if let Ok(poll) = tex {
            if let Some(id) = poll.texture_id() {
                let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));

                let mut mesh = Mesh::with_texture(id);
                mesh.add_rect_with_uv(area, uv, Color32::WHITE);
                mesh.rotate(rot, area.min + Vec2 { x: 0.5, y: 0.5 } * area.size());
                return Shape::mesh(mesh);
            }
        } else if let Err(e) = tex {
            log::log!(log::Level::Error, "{}", e);
        }
    }
    Shape::Rect(RectShape::new(
        area.scale_from_center(scale),
        Rounding::ZERO,
        Color32::DARK_RED,
        Stroke::NONE,
    ))
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
            Stroke::new((self.paradox.0 as f32 / 5.0) - 1.0, Color32::WHITE),
        )));

        if let Some(building) = self.building {
            ret.push(object_to_shape(
                ctx,
                &building.definition.appearance,
                area,
                0.9,
                AbsoluteDirection::S,
            ));
        }

        if let Some(item) = self.items[0] {
            ret.push(object_to_shape(
                ctx,
                &item.definition.appearance,
                area,
                0.8,
                AbsoluteDirection::S,
            ));
        }

        if let Some(actor) = self.actor {
            ret.push(object_to_shape(
                ctx,
                &actor.descriptor.appearance,
                area,
                0.8,
                actor.facing,
            ));
        }
        ret
    }
}

impl WorldWindowWidget<'_> {
    pub fn paint(self, ctx: &egui::Context, area: Rect) -> Vec<Shape> {
        let size = area.size();
        let cell_size = Vec2 { x: 32.0, y: 32.0 };

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
        if self.show_cursor {
            ret.push(Shape::Circle(CircleShape {
                //Ugly but easy to implement center-finder. Use Centerx
                center: Pos2::new((size.x + cell_size.x) / 2.0, (size.y - cell_size.y) / 2.0),
                radius: 20.0,
                fill: Color32::TRANSPARENT,
                stroke: Stroke::new(5.0, Color32::DEBUG_COLOR),
            }));
        }
        ret
    }
}
