extern crate embedded_graphics;

use rppal::i2c::I2c;
use ssd1306::{Builder, mode::GraphicsMode, interface::i2c::I2cInterface};
use embedded_graphics::fonts::{Font,Font6x8};
use embedded_graphics::prelude::*;
use embedded_graphics::image::Image1BPP;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::PixelColorU8;
use embedded_graphics::Drawing;

const DEFAULT_WIDTH: i32 = 128;
const DEFAULT_HEIGHT: i32 = 64;

pub struct Screen {
    pub display: GraphicsMode<I2cInterface<I2c>>,
    width: i32,
    height: i32
}

impl Screen {
    pub fn new() -> Screen {
        let mut i2c = I2c::new().expect("Could not create I2C Device");
        i2c.set_slave_address(0x3c).expect("Could not select device");

        Screen {
            display: Builder::new().connect_i2c(i2c).into(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT
        }
    }

    pub fn set_shape(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_width(&self) -> i32 { self.width }
    pub fn get_height(&self) -> i32 { self.height }

    pub fn clear(&mut self) {
        self.display.clear()
    }
    pub fn flush(&mut self) {
        self.display.flush().unwrap()
    }
}
