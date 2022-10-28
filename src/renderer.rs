use crate::buttons::ButtonSet;
use crate::view::{View,UpdateResult::*};
use crate::screen::Screen;

pub struct Renderer {
    views: Vec<Box<dyn View>>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            views: Vec::new(),
        }
    }

    pub fn push_view(&mut self, view: Box<dyn View>) {
        self.views.push(view)
    }

    pub fn pop_view(&mut self) {
        self.views.pop();
    }

    pub fn render(&mut self, screen: &mut Screen) {
        match self.views.last() {
            Some(view) => view.render(screen),
            None => { }
        }
    }

    pub fn update(&mut self, buttons: &mut ButtonSet) {
        buttons.poll_all();

        let result_option = match self.views.last_mut() {
            Some(view) => view.update(buttons),
            None => { None }
        };

        if let Some(result) = result_option {
            match result {
                NewView(view) => self.push_view(view),
                Back(None) => { if self.views.len() > 1 { self.pop_view(); }; }
                Back(Some(result)) => {
                    if self.views.len() > 1 {
                        self.pop_view();
                        if let Some(view) = self.views.last_mut() {
                            view.result(result.get());
                        }
                    }
                }
            };
        }

        buttons.flush();
    }
}
