use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub struct KeyController {
    pub mode: char,
    tree: HashMap<char, KeyTree>,
    current: Option<KeyTree>,
}

impl KeyController {
    pub fn new(keymap: HashMap<(char, KeyModifiers, Vec<KeyCode>, Vec<char>), String>) -> Self {
        let mut tree = HashMap::new();
        tree.insert(
            'n',
            KeyTree::new(
                KeyModifiers::NONE,
                vec![KeyCode::Char('y'), KeyCode::Char('y')],
                "yank".into(),
            ),
        );
        tree.insert(
            'n',
            KeyTree::new(
                KeyModifiers::NONE,
                vec![KeyCode::Char('d'), KeyCode::Char('d')],
                "delete".into(),
            ),
        );
        tree.insert(
            'n',
            KeyTree::new(
                KeyModifiers::NONE,
                vec![KeyCode::Char('R')],
                "replace".into(),
            ),
        );
        tree.insert(
            'n',
            KeyTree::new(
                KeyModifiers::CONTROL,
                vec![KeyCode::Char('f')],
                "find".into(),
            ),
        );
        tree.insert(
            'n',
            KeyTree::new(
                KeyModifiers::NONE,
                vec![KeyCode::Char('i')],
                "insert".into(),
            ),
        );
        Self {
            mode: 'n',
            current: tree.get(&'n').cloned(),
            tree,
        }
    }
    pub fn process(&mut self, key: KeyEvent) -> Option<String> {
        if let Some(tree) = &self.current {
            match tree.0.get(&(key.code, key.modifiers)) {
                Some(KeyTreeOption::End(s)) => Some(s.clone()),
                Some(KeyTreeOption::Tree(kt)) => {
                    self.current = Some(kt.as_ref().clone());
                    None
                }
                _ => None,
            }
        } else {
            self.current = self.tree.get(&self.mode).cloned();
            if self.current.is_some() {
                self.process(key)
            } else {
                None
            }
        }
    }
    pub fn mode_name(&self) -> &'static str {
        match self.mode {
            'n' => "NORMAL",
            'i' => "INSERT",
            'v' => "VISUAL",
            'l' => "V-LINE",
            'b' => "V-BLOCK",
            _ => "      ",
        }
    }
}

#[derive(Debug, Clone)]
enum KeyTreeOption {
    Tree(Box<KeyTree>),
    End(String),
}

#[derive(Debug, Clone)]
struct KeyTree(HashMap<(KeyCode, KeyModifiers), KeyTreeOption>);

impl KeyTree {
    pub fn new(mods: KeyModifiers, mut v: Vec<KeyCode>, end: String) -> Self {
        let first_key = v.remove(0);
        let o = if v.len() == 0 {
            KeyTreeOption::End(end)
        } else {
            KeyTreeOption::Tree(Box::new(Self::new(mods, v, end)))
        };
        Self(HashMap::from([((first_key, mods), o)]))
    }
}
