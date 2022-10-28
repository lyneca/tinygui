use crate::{screen::Screen, shape::Drawable};
use embedded_graphics::prelude::*;
use embedded_graphics::coord::Coord;
use embedded_graphics::image::Image1BPP;

pub struct Bitmap {
    x: i32, y: i32,
    width: i32, height: i32,
    image: Vec<u8>
}

impl Bitmap {
    pub fn new(image: &[u8], width: i32, height: i32) -> Bitmap {
        Bitmap {
            x: 0, y: 0,
            width, height,
            image: Vec::from(image)
        }
    }

    pub fn invert(mut self, invert: bool) -> Bitmap {
        self.image = self.image.iter()
                               .map(|row| match invert { true => !row, false => *row })
                               .collect();
        self
    }

    pub fn at(mut self, x: i32, y: i32) -> Bitmap { self.x = x; self.y = y; self }
}

impl<'a> Drawable for Bitmap {
    fn draw(&self, screen: &mut Screen) { self.draw_at(self.x, self.y, screen) }
    fn draw_at(&self, x: i32, y: i32, screen: &mut Screen) {
        screen.display.draw(
            Image1BPP::new(self.image.as_slice(), self.width as u32, self.height as u32)
            .translate(Coord::new(x, y))
            .into_iter());
    }
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }
}

