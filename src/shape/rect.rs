use crate::{screen::Screen, shape::{to_color, Drawable}};
use embedded_graphics::prelude::*;
use embedded_graphics::coord::Coord;
use embedded_graphics::primitives;

pub struct Rect {
    x: i32, y: i32,
    width: i32, height: i32,
    fill: Option<u8>,
    stroke: Option<u8>,
    thickness: u8
}

impl Rect {
    pub fn new(width: i32, height: i32) -> Rect {
        Rect {
            x: 0, y: 0,
            width, height,
            fill: None,
            stroke: Some(1),
            thickness: 1
        }
    }

    pub fn at(mut self, x: i32, y: i32) -> Rect { self.x = x; self.y = y; self }
    pub fn stroke(mut self, stroke: Option<u8>) -> Rect { self.stroke = stroke; self }
    pub fn fill(mut self, fill: Option<u8>) -> Rect { self.fill = fill; self }
    pub fn thickness(mut self, thickness: u8) -> Rect { self.thickness = thickness; self }
}

impl Drawable for Rect {
    fn draw(&self, screen: &mut Screen) { self.draw_at(self.x, self.y, screen) }
    fn draw_at(&self, x: i32, y: i32, screen: &mut Screen) {
        screen.display.draw(
            primitives::Rect::new(
                Coord::new(x, y),
                Coord::new(x + self.width, y + self.height))
            .with_stroke(to_color(self.stroke))
            .with_fill(to_color(self.fill))
            .into_iter());
    }
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }
}
