use crate::{screen::Screen, shape::{to_color, Drawable}};
use embedded_graphics::prelude::*;
use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::{Font,Font6x8,font_builder::{FontBuilderConf,FontBuilderIterator}};

pub struct Text {
    x: i32, y: i32,
    text: String,
    fill: Option<u8>,
    stroke: Option<u8>,
}

impl Text {
    pub fn new(text: String) -> Text {
        Text {
            x: 0, y: 0,
            text,
            fill: None,
            stroke: Some(1),
        }
    }

    pub fn set_text(mut self, text: String) -> Text { self.text = text; self }
    pub fn at(mut self, x: i32, y: i32) -> Text { self.x = x; self.y = y; self }
    pub fn stroke(mut self, stroke: Option<u8>) -> Text { self.stroke = stroke; self }
    pub fn fill(mut self, fill: Option<u8>) -> Text { self.fill = fill; self }
}

impl Drawable for Text {
    fn draw(&self, screen: &mut Screen) { self.draw_at(self.x, self.y, screen) }
    fn draw_at(&self, x: i32, y: i32, screen: &mut Screen) {
        screen.display.draw(
            Font6x8::render_str(self.text.as_str())
            .with_stroke(to_color(self.stroke))
            .with_fill(to_color(self.fill))
            .translate(Coord::new(x, y))
            .into_iter());
    }
    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }
}
