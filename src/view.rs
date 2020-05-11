use crate::screen::Screen;
use crate::buttons::ButtonSet;

pub trait View {
    fn update(&mut self, buttons: &mut ButtonSet);
    fn render(&self, screen: &mut Screen);
}
