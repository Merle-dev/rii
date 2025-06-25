use std::{
    collections::HashMap,
    io::{Result, Stdout, Write, stdout},
};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{PrintStyledContent, Stylize},
    terminal::{
        DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use glam::IVec2;

use crate::keys::KeyController;

#[derive(Clone, Debug)]
struct Sign {
    char: &'static str,
    mode: u8,
    fg_color: u8,
    bg_color: u8,
}

impl Sign {
    fn new(char: &'static str) -> Self {
        Self {
            char,
            mode: 0,
            fg_color: 0,
            bg_color: 0,
        }
    }
}

pub struct App {
    stdout: Stdout,
    size: (u16, u16),
    cursor: IVec2,
    scroll: i32,

    buffer: Vec<Vec<Sign>>,
    keycontroller: KeyController,
    update_lines: Vec<usize>,
}
impl App {
    pub fn new(
        keymap: HashMap<(char, KeyModifiers, Vec<KeyCode>, Vec<char>), String>,
    ) -> Result<Self> {
        let mut stdout = stdout();
        let size = crossterm::terminal::size()?;
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, DisableLineWrap)?;
        Ok(Self {
            stdout,
            size,
            cursor: IVec2::ZERO,
            scroll: 0,

            buffer: vec![vec![Sign::new(" "); size.0 as usize]; size.1 as usize],
            keycontroller: KeyController::new(keymap),
            update_lines: vec![],
        })
    }
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.draw()?;
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Esc => break,
                    _ => {
                        if let Some(line) = self.keycontroller.process(key) {
                            self.new_simple_line(
                                10,
                                line.chars()
                                    .map(|c| Sign::new(Box::leak(c.to_string().into_boxed_str())))
                                    .collect(),
                            );
                        }
                    }
                },
                _ => (),
            };
        }
        Ok(())
    }

    pub fn new_simple_line(&mut self, index: usize, vec: Vec<Sign>) {
        self.update_lines.push(index);
        self.buffer[index] = vec;
    }

    pub fn draw(&mut self) -> Result<()> {
        for n in self.update_lines.iter() {
            let line = self.buffer[*n]
                .iter()
                .map(|s| s.char)
                .collect::<Vec<&'static str>>()
                .concat();
            self.stdout
                .queue(MoveTo(0, *n as u16))?
                .queue(PrintStyledContent(line.white()))?;
        }
        self.update_lines = vec![];
        self.stdout
            .queue(MoveTo(self.cursor.x as u16, self.cursor.y as u16))?
            .flush()?;
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
