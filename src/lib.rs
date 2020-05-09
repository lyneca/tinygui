mod view;
mod screen;
mod renderer;
mod input_handler;

use rppal::i2c::I2c;

use renderer::Renderer;
use screen::Screen;
use input_handler::InputHandler;

pub struct GUI {
    renderer: Renderer,
    screen: Screen,
    input_handler: InputHandler
}

impl GUI {
    fn run(&mut self) {
        loop {
            self.screen.clear();
            self.renderer.render(&mut self.screen);
            self.screen.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
