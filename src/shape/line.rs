use crate::{screen::Screen, shape::{to_color, Drawable}};
use embedded_graphics::prelude::*;
use embedded_graphics::coord::Coord;
use embedded_graphics::primitives;

pub struct Line {
    x: i32, y: i32,
    width: i32, height: i32,
    stroke: Option<u8>,
    thickness: u8
}

impl Line {
    pub fn new(width: i32, height: i32) -> Line {
        Line {
            x: 0, y: 0,
            width, height,
            stroke: Some(1),
            thickness: 1
        }
    }

    pub fn at(mut self, x: i32, y: i32) -> Line { self.x = x; self.y = y; self }
    pub fn stroke(mut self, stroke: Option<u8>) -> Line { self.stroke = stroke; self }
    pub fn thickness(mut self, thickness: u8) -> Line { self.thickness = thickness; self }
}

impl Drawable for Line {
    fn draw(&self, screen: &mut Screen) { self.draw_at(self.x, self.y, screen) }
    fn draw_at(&self, x: i32, y: i32, screen: &mut Screen) {
        screen.display.draw(
            primitives::Line::new(
                Coord::new(x, y),
                Coord::new(x + self.width, y + self.height))
            .with_stroke(to_color(self.stroke))
            .into_iter());
    }
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }
}

