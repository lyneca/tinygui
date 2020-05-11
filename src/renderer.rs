use crate::buttons::ButtonSet;
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

    pub fn add_view(&mut self, view: Box<dyn View>) {
        self.views.push(view)
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

    pub fn update(&mut self, buttons: &mut ButtonSet) {
        buttons.poll_all();
        match self.views.get_mut(self.current) {
            Some(view) => view.update(buttons),
            None => {
                self.current = 0;
                if let Some(view) = self.views.get_mut(0) {
                    view.update(buttons)
                }
            }
        }
        buttons.flush();
    }
}
