use std::{
    collections::HashMap,
    time::{self, Duration, Instant},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct KeyController {
    mode: char,
    keymap: HashMap<(char, KeyModifiers, Vec<KeyCode>, Vec<char>), String>,
    backload: (Vec<KeyCode>, Vec<char>),
    last_time: Instant,
}

impl KeyController {
    pub fn new(keymap: HashMap<(char, KeyModifiers, Vec<KeyCode>, Vec<char>), String>) -> Self {
        Self {
            mode: 'n',
            keymap,
            backload: (vec![], vec![]),
            last_time: Instant::now(),
        }
    }
    pub fn process(&mut self, key: KeyEvent) -> Option<String> {
        match key.code {
            KeyCode::Char(c) => self.backload.1.push(c),
            _ => self.backload.0.push(key.code),
        };
        let now = Instant::now();
        let mut r = None;
        if now - self.last_time < Duration::from_millis(600) {
            r = self.keymap.get(&(
                self.mode,
                key.modifiers,
                self.backload.0.clone(),
                self.backload.1.clone(),
            ));
            println!("{:?} | {:?}", self.backload, r);
            if r.is_some() {
                self.backload = (vec![], vec![]);
            }
            self.last_time = now;
        } else {
            println!("{:?}", self.backload);
            self.backload = (vec![], vec![]);
        }
        self.last_time = now;
        r.cloned()
    }
}
