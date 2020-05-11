pub mod view;
pub mod views;
pub mod shape;
pub mod screen;
pub mod renderer;
pub mod buttons;

use renderer::Renderer;
use screen::Screen;
use buttons::ButtonSet;

pub struct GUI {
    pub renderer: Renderer,
    pub screen: Screen,
    pub buttons: ButtonSet
}

impl GUI {
    pub fn new() -> GUI {
        GUI {
            renderer: Renderer::new(),
            screen: Screen::new(),
            buttons: ButtonSet::default_pins()
        }
    }
    pub fn run(&mut self) {
        self.screen.display.init().expect("Could not initialise screen.");
        loop {
            self.screen.clear();
            self.renderer.update(&mut self.buttons);
            self.renderer.render(&mut self.screen);
            self.screen.flush();
        }
    }
}
