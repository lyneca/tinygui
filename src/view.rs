use crate::screen::Screen;
use crate::input_handler::Input;

pub trait View {
    fn update(&mut self, input: Input);
    fn render(&self, screen: &mut Screen);
}
