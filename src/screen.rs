extern crate embedded_graphics;

use rppal::i2c::I2c;
use ssd1306::{Builder, mode::GraphicsMode, interface::i2c::I2cInterface};
use embedded_graphics::fonts::{Font,Font6x8};
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::image::Image1BPP;
use embedded_graphics::primitives::Rect;
use embedded_graphics::primitives::Line;
use embedded_graphics::Drawing;

pub struct Screen {
    display: GraphicsMode<I2cInterface<I2c>>
}

impl Screen {
    pub fn new() -> Screen {
        let mut i2c = I2c::new().expect("Could not create I2C Device");
        i2c.set_slave_address(0x3c).expect("Could not select device");

        Screen {
            display: Builder::new().connect_i2c(i2c).into()
        }
    }
    pub fn draw_rect(&mut self, x: u8, y: u8, width: u8, height: u8) {
        self.display.draw(Rect::new(
                Coord::new(x as i32, y as i32),
                Coord::new((x + width) as i32, (y + height) as i32))
            .into_iter())
    }

    pub fn clear(&mut self) {
        self.display.clear()
    }
    pub fn flush(&mut self) {
        self.display.flush().unwrap()
    }
}
