use std::any::Any;

use crate::view::{View, UpdateResult};
use crate::screen::Screen;
use crate::buttons::ButtonSet;
use crate::shape::{Drawable,Text,Section};

pub struct HelloWorld {
    root: Box<dyn Drawable>
}

impl HelloWorld {
    pub fn new(screen: &Screen) -> HelloWorld { 
        let mut main = Section::new(screen.get_width() - 1, screen.get_height() - 1)
            .stroke(Some(1))
            .at(0, 0);
        let text = Text::new("Hello, world!".to_owned()).at(3, 3);
        main.push(Box::new(text));
        HelloWorld {
            root: Box::new(main)
        }
    }
}

impl View for HelloWorld {
    fn update(&mut self, buttons: &mut ButtonSet) -> Option<UpdateResult> { None }
    fn render(&self, screen: &mut Screen) {
        self.root.draw(screen);
    }
    fn result(&mut self,  result: Box<dyn Any>) {}
}


