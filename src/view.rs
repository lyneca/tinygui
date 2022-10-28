use std::any::Any;

use crate::screen::Screen;
use crate::buttons::ButtonSet;

pub trait ViewResult {
    fn get(&self) -> Box<dyn Any>;
}

pub enum UpdateResult {
    NewView(Box<dyn View>),
    Back(Option<Box<dyn ViewResult>>)
}

pub trait View {
    fn update(&mut self, buttons: &mut ButtonSet) -> Option<UpdateResult> {
        if buttons.b.was_pressed() {
            Some(UpdateResult::Back(None))
        } else {
            None
        }
    }
    fn result(&mut self,  result: Box<dyn Any>);
    fn render(&self, screen: &mut Screen);
}

pub trait ViewSpawner {
    fn spawn(&self) -> Box<dyn View>;
}


