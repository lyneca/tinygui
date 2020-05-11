use crate::{screen::Screen, shape::{Drawable, Rect}};

pub struct Section {
    x: i32, y: i32,
    width: i32, height: i32,
    fill: Option<u8>,
    stroke: Option<u8>,
    thickness: u8,
    contents: Vec<Box<dyn Drawable>>
}

impl Section {
    pub fn new(width: i32, height: i32) -> Section {
        Section {
            x: 0, y: 0,
            width, height,
            fill: None,
            stroke: None,
            thickness: 0,
            contents: vec![]
        }
    }

    pub fn push(&mut self, other: Box<dyn Drawable>) {
        self.contents.push(other);
    }

    pub fn at(mut self, x: i32, y: i32) -> Section { self.x = x; self.y = y; self }
    pub fn stroke(mut self, stroke: Option<u8>) -> Section { self.stroke = stroke; self }
    pub fn fill(mut self, fill: Option<u8>) -> Section { self.fill = fill; self }
}

impl Drawable for Section {
    fn draw(&self, screen: &mut Screen) { self.draw_at(self.x, self.y, screen) }
    fn draw_at(&self, x: i32, y: i32, screen: &mut Screen) {
        for view in self.contents.iter() {
            view.draw_at(view.get_x() + x, view.get_y() + y, screen);
        }
        Rect::new(self.width, self.height)
            .at(x, y)
            .fill(self.fill)
            .stroke(self.stroke)
            .thickness(self.thickness)
            .draw(screen)
    }

    fn get_x(&self) -> i32 { self.x }
    fn get_y(&self) -> i32 { self.y }
}
