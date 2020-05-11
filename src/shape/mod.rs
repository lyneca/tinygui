mod rect;
mod bitmap;
mod line;
mod text;
mod section;

pub use rect::Rect;
pub use bitmap::Bitmap;
pub use section::Section;
pub use text::Text;
pub use line::Line;

use embedded_graphics::pixelcolor::PixelColorU8;

use crate::screen::Screen;

pub fn to_color(color: Option<u8>) -> Option<PixelColorU8>{
    match color {
        Some(c) => Some(PixelColorU8(c)),
        None => None
    }
}

pub trait Drawable {
    fn draw(&self, screen: &mut Screen);
    fn draw_at(&self, x: i32, y: i32, screen: &mut Screen);
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}
