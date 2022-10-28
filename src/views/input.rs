use std::any::Any;

use crate::view::{View, ViewSpawner, UpdateResult, ViewResult};
use crate::screen::Screen;
use crate::buttons::{ButtonSet};
use crate::shape::{Drawable,Text,Rect, Bitmap, Line};

pub enum Key {
    Letter(char),
    Submit,
    Cancel
}

pub struct TextInputSpawner {

}

impl ViewSpawner for TextInputSpawner {
    fn spawn(&self) -> Box<dyn View> {
        Box::new(TextInput::new())
    }
}

pub struct TextInput {
    normal: Vec<Vec<Key>>,
    shift: Vec<Vec<Key>>,
    x: i32,
    y: i32,
    shifting: bool,
    string: String
}

impl TextInput {
    pub fn new() -> TextInput {
        TextInput {
            normal: vec![
                vec![Key::Letter('1'), Key::Letter('2'), Key::Letter('3'), Key::Letter('4'), Key::Letter('5'), Key::Letter('6'), Key::Letter('7'), Key::Letter('8'), Key::Letter('9'), Key::Letter('0'), Key::Letter('-'), Key::Letter('=')],
                vec![Key::Letter('q'), Key::Letter('w'), Key::Letter('e'), Key::Letter('r'), Key::Letter('t'), Key::Letter('y'), Key::Letter('u'), Key::Letter('i'), Key::Letter('o'), Key::Letter('p'), Key::Letter('['), Key::Letter(']')],
                vec![Key::Letter('a'), Key::Letter('s'), Key::Letter('d'), Key::Letter('f'), Key::Letter('g'), Key::Letter('h'), Key::Letter('j'), Key::Letter('k'), Key::Letter('l'), Key::Letter(';'), Key::Letter('\''), Key::Submit],
                vec![Key::Letter('z'), Key::Letter('x'), Key::Letter('c'), Key::Letter('v'), Key::Letter('b'), Key::Letter('n'), Key::Letter('m'), Key::Letter(','), Key::Letter('.'), Key::Letter('/'), Key::Letter('\\'), Key::Letter(' ')]
            ],
            shift: vec![
                vec![Key::Letter('!'), Key::Letter('@'), Key::Letter('#'), Key::Letter('$'), Key::Letter('%'), Key::Letter('^'), Key::Letter('&'), Key::Letter('*'), Key::Letter('('), Key::Letter(')'), Key::Letter('_'), Key::Letter('+')],
                vec![Key::Letter('Q'), Key::Letter('W'), Key::Letter('E'), Key::Letter('R'), Key::Letter('T'), Key::Letter('Y'), Key::Letter('U'), Key::Letter('I'), Key::Letter('O'), Key::Letter('P'), Key::Letter('{'), Key::Letter('}')],
                vec![Key::Letter('A'), Key::Letter('S'), Key::Letter('D'), Key::Letter('F'), Key::Letter('G'), Key::Letter('H'), Key::Letter('J'), Key::Letter('K'), Key::Letter('L'), Key::Letter(':'), Key::Letter('"'), Key::Cancel],
                vec![Key::Letter('Z'), Key::Letter('X'), Key::Letter('C'), Key::Letter('V'), Key::Letter('B'), Key::Letter('N'), Key::Letter('M'), Key::Letter('<'), Key::Letter('>'), Key::Letter('?'), Key::Letter('|'), Key::Letter(' ')]
            ],
            x: 0, y: 0,
            shifting: false,
            string: "".to_owned()
        }
    }
    pub fn spawner() -> TextInputSpawner {
        TextInputSpawner {  }
    }
}

const SUBMIT_ICON: &[u8] = &[
    0b00000000,
    0b00000100,
    0b00001000,
    0b00010000,
    0b10100000,
    0b01000000,
    0b00000000,
    0b11111100,
];
const CANCEL_ICON: &[u8] = &[
    0b10000100,
    0b01001000,
    0b00110000,
    0b00110000,
    0b01001000,
    0b10000100,
    0b00000000,
    0b11111100,
];
const SPACE_ICON: &[u8] = &[
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b00000000,
    0b10000100,
    0b11111100,
];

const KEYBOARD_OFFSET: i32 = 15;

pub struct TextResult {
    text: String
}

impl ViewResult for TextResult {
    fn get(&self) -> Box<dyn Any> {
        Box::new(self.text.clone())
    }
}

impl View for TextInput {
    fn update(&mut self, buttons: &mut ButtonSet) -> Option<UpdateResult> {
        let map = match self.shifting {
            false => &self.normal,
            true => &self.shift
        };
        if buttons.b.was_pressed() {
            if self.string.len() > 0 {
                self.string.pop();
            }
        }
        if buttons.c.was_pressed() {
            self.shifting = !self.shifting;
        }
        if buttons.up.was_pressed() {
            self.y -= 1;
            if self.y < 0 { self.y = (map.len() - 1) as i32 } 
        }
        if buttons.down.was_pressed() {
            self.y += 1;
            self.y %= map.len() as i32;
        }
        if buttons.left.was_pressed() {
            self.x -= 1;
            if self.x < 0 { self.x = (map[self.y as usize].len() - 1) as i32 } 
        }
        if buttons.right.was_pressed() {
            self.x += 1;
            self.x %= map[self.y as usize].len() as i32;
        }
        if buttons.a.was_pressed() {
            return match map.get(self.y as usize).unwrap().get(self.x as usize).unwrap() {
                Key::Submit => Some(UpdateResult::Back(Some(Box::new(TextResult { text: self.string.clone() })))),
                Key::Cancel => Some(UpdateResult::Back(None)),
                Key::Letter(letter) => {
                    self.string.push(*letter);
                    return None;
                }
            }
        }
        None
    }

    fn render(&self, screen: &mut Screen) {
        let map = match self.shifting {
            false => &self.normal,
            true => &self.shift
        };
        for y in 0..map.len() as i32 {
            for x in 0..map[y as usize].len() as i32 {
                Rect::new(10, 12)
                    .at(10 * x, KEYBOARD_OFFSET + 12 * y)
                    .fill(Some((x == self.x && y == self.y) as u8))
                    .stroke(Some(1))
                    .draw(screen);
                match map[y as usize][x as usize] {
                    Key::Submit => Bitmap::new(SUBMIT_ICON, 6, 8)
                                    .at(10 * x + 2, KEYBOARD_OFFSET + 12 * y + 2)
                                    .invert(x == self.x && y == self.y)
                                    .draw(screen),
                    Key::Cancel => Bitmap::new(CANCEL_ICON, 6, 8)
                                    .at(10 * x + 2, KEYBOARD_OFFSET + 12 * y + 2)
                                    .invert(x == self.x && y == self.y)
                                    .draw(screen),
                    Key::Letter(' ') => Bitmap::new(SPACE_ICON, 6, 8)
                                    .at(10 * x + 2, KEYBOARD_OFFSET + 12 * y + 2)
                                    .invert(x == self.x && y == self.y)
                                    .draw(screen),
                    Key::Letter(letter) => Text::new(letter.to_string())
                                    .at(10 * x + 3, KEYBOARD_OFFSET + 12 * y + 3)
                                    .stroke(Some(!(x == self.x && y == self.y) as u8))
                                    .fill(Some((x == self.x && y == self.y) as u8))
                                    .draw(screen)
                }
            }
        }
        Line::new(screen.get_width() - 4, 0)
            .at(2, 11)
            .draw(screen);
        Text::new(self.string.clone()).at(2, 2).draw(screen);
    }
    fn result(&mut self, result: Box<dyn Any>) { }
}