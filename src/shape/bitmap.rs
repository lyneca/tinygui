use crate::{screen::Screen, shape::{to_color, Drawable}};
use embedded_graphics::prelude::*;
use embedded_graphics::coord::Coord;
use embedded_graphics::image::Image1BPP;

pub struct Bitmap<'a> {
    x: i32, y: i32,
    width: i32, height: i32,
    image: &'a [u8]
}

impl<'a> Bitmap<'a> {
    pub fn new(image: &'a [u8], width: i32, height: i32) -> Bitmap<'a> {
        Bitmap {
            x: 0, y: 0,
            width, height,
            image: image
        }
    }

    pub fn at(mut self, x: i32, y: i32) -> Bitmap<'a> { self.x = x; self.y = y; self }
}

impl<'a> Drawable for Bitmap<'a> {
    fn draw(&self, screen: &mut Screen) { self.draw_at(self.x, self.y, screen) }
    fn draw_at(&self, x: i32, y: i32, screen: &mut Screen) {
        screen.display.draw(
            Image1BPP::new(self.image, self.width as u32, self.height as u32)
            .translate(Coord::new(x, y))
            .into_iter());
    }
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }
}

