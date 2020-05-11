use crate::view::View;
use crate::screen::Screen;
use crate::buttons::ButtonSet;
use crate::shape::{Drawable,Text,Rect,Line,Bitmap};
use std::cmp::max;
use embedded_graphics::Drawing;

const ENTRY_HEIGHT: i32 = 13;

const ARROW_DOWN: &[u8] = &[
    0b00000000,
    0b00100000,
    0b00100000,
    0b00100000,
    0b10101000,
    0b01110000,
    0b00100000,
    0b00000000,
];

pub trait CustomMenuEntry {
    fn render(&self, x: i32, y: i32, width: i32, height: i32, selected: bool, screen: &mut Screen);
    fn activate(&self) -> Option<&Box<dyn View>>;
}

pub enum MenuItem {
    Custom(Box<dyn CustomMenuEntry>),
    TextToView(String, Box<dyn View>),
    TextToFunc(String, Box<dyn Fn()>)
}

pub struct Menu {
    entries: Vec<MenuItem>,
    title: String,
    selected: usize,
    first_shown: usize
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            entries: vec![],
            title: String::new(),
            selected: 0,
            first_shown: 0
        }
    }

    pub fn title(mut self, title: String) -> Menu {
        self.title = title;
        self
    }

    pub fn add_entry(&mut self, item: MenuItem) {
        self.entries.push(item);
    }
}

impl View for Menu {
    fn update(&mut self, buttons: &mut ButtonSet) {
        // navigating the menu
        if buttons.down.was_pressed() {
            if self.selected == self.entries.len() - 1 {
                self.selected = 0;
            } else {
                self.selected += 1;
            }
        }

        if buttons.up.was_pressed() {
            if self.selected == 0 {
                self.selected = self.entries.len() - 1;
            } else {
                self.selected -= 1;
            }
        }

        // skipping to the start/end of the menu
        if buttons.left.was_pressed() {
            self.selected = 0;
        }

        if buttons.right.was_pressed() {
            self.selected = self.entries.len() - 1;
        }

        if self.selected == self.entries.len() - 1 {
            self.first_shown = max(0, self.selected - 3);
        } else if self.selected == 0 {
            self.first_shown = 0;
        } else if self.selected >= self.first_shown + 2 {
            while self.first_shown < self.selected - 2 {
                self.first_shown += 1;
            }
        } else if self.selected <= self.first_shown {
            while self.first_shown > self.selected - 1 {
                self.first_shown -= 1;
            }
        }

        // selecting options
        if buttons.a.was_pressed() {
            use MenuItem::*;
            match &self.entries[self.selected] {
                Custom(entry) => entry.activate(),
                TextToView(_, view) => Some(view),
                TextToFunc(_, func) => {
                    func();
                    None
                }
            };
        }
    }

    fn render(&self, screen: &mut Screen) {
        // draw entries
        for (i, entry) in self.entries.iter().skip(self.first_shown).take(4).enumerate() {
            use MenuItem::*;
            let is_selected = std::ptr::eq(&self.entries[self.selected as usize], entry);
            match entry {
                Custom(entry) => {
                    entry.render(
                        (i as i32) * ENTRY_HEIGHT, 0,
                        screen.get_width(), ENTRY_HEIGHT,
                        is_selected, screen
                    )
                }
                TextToView(text, _) | TextToFunc(text, _) => {
                    Rect::new(screen.get_width() - 1, ENTRY_HEIGHT)
                        .fill(Some(is_selected as u8))
                        .at(0, i as i32 * ENTRY_HEIGHT)
                        .draw(screen);
                    Text::new(text.clone())
                        .at(3, i as i32 * ENTRY_HEIGHT + 3)
                        .fill(Some(is_selected as u8))
                        .stroke(Some(!is_selected as u8))
                        .draw(screen);
                }
            }
        }
        // draw a line in case there are less than 4 options
        Line::new(screen.get_width() - 1, 0)
            .at(0, 13 * 4)
            .draw(screen);

        // draw arrow
        Bitmap::new(ARROW_DOWN, 5, 8)
            .at(screen.get_width() - 8, 4 * 13 + 2)
            .draw(screen);

        // draw boundary rectangle
        Rect::new(screen.get_width() - 1, screen.get_height() - 1)
            .at(0, 0)
            .draw(screen);
    }
}



