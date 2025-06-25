use std::{collections::HashMap, io::Result};

mod app;
mod keys;

use app::*;
use crossterm::event::{KeyCode, KeyModifiers};

fn main() -> Result<()> {
    // let args: Vec<String> = std::env::args().collect();

    let mut keymap: HashMap<(char, KeyModifiers, Vec<KeyCode>, Vec<char>), String> = HashMap::new();

    keymap.insert(
        ('n', KeyModifiers::NONE, vec![], vec!['y', 'y']),
        "Yeet".into(),
    );
    keymap.insert(
        ('n', KeyModifiers::NONE, vec![], vec!['i']),
        "Insert".into(),
    );
    keymap.insert(
        ('n', KeyModifiers::CONTROL, vec![], vec!['r']),
        "Replace".into(),
    );

    let mut app = App::new(keymap)?;
    app.run()?;
    Ok(())
}
