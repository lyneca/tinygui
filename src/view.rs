use crate::screen::Screen;
use crate::buttons::ButtonSet;

pub enum UpdateResult {
    NewView(Box<dyn View>),
    Back
}

pub trait View {
    fn update(&mut self, buttons: &mut ButtonSet) -> Option<UpdateResult> {
        if buttons.b.was_pressed() {
            Some(UpdateResult::Back)
        } else {
            None
        }
    }
    fn render(&self, screen: &mut Screen);
}

pub trait ViewSpawner {
    fn spawn(&self) -> Box<dyn View>;
}


