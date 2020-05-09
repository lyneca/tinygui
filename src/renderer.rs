use crate::input_handler::InputHandler;
use crate::view::View;
use crate::screen::Screen;

pub struct Renderer {
    views: Vec<Box<dyn View>>,
    current: usize
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            views: Vec::new(),
            current: 0
        }
    }

    pub fn render(&mut self, screen: &mut Screen) {
        match self.views.get(self.current) {
            Some(view) => view.render(screen),
            None => {
                self.current = 0;
                if let Some(view) = self.views.get(0) {
                    view.render(screen)
                }
            }
        }
    }

    pub fn update(&mut self, input_handler: &mut InputHandler) {
        let input = input_handler.get_input();
        match self.views.get_mut(self.current) {
            Some(view) => view.update(input),
            None => {
                self.current = 0;
                if let Some(view) = self.views.get_mut(0) {
                    view.update(input)
                }
            }
        }
    }
}
