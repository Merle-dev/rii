use std::{
    collections::HashMap,
    io::{Result, Stdout, Write, stdout},
};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{
        DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use glam::IVec2;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    keys::KeyController,
    render::{self, Render, WindowType},
};

#[derive(Clone, Debug)]
pub struct Sign {
    char: String,
    mode: u8,
    fg_color: Color,
    bg_color: Color,
}

impl Sign {
    pub fn new(char: String) -> Self {
        Self {
            char,
            mode: 0,
            fg_color: Color::White,
            bg_color: Color::Reset,
        }
    }
}

pub struct App {
    stdout: Stdout,
    size: (u16, u16),
    pub cursor: IVec2,
    scroll: i32,

    pub buffer: Vec<Vec<Sign>>,
    pub keycontroller: KeyController,
    pub render: Render,
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

            buffer: vec![vec![Sign::new(" ".into()); size.0 as usize]; size.1 as usize],
            keycontroller: KeyController::new(keymap),
            render: Render::new(size),
            update_lines: vec![],
        })
    }
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.draw()?;
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Esc => break,
                    _ => if let Some(line) = self.keycontroller.process(key) {},
                },
                _ => (),
            };
        }
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        self.render_footer();
        for n in self.update_lines.iter() {
            for sign in self.buffer[*n].iter() {
                self.stdout.queue(PrintStyledContent(
                    sign.char.clone().with(sign.fg_color).on(sign.bg_color),
                ))?;
            }
        }
        self.update_lines = vec![];
        self.stdout
            .queue(MoveTo(self.cursor.x as u16, self.cursor.y as u16))?
            .flush()?;
        Ok(())
    }

    pub fn render(&mut self, wtype: WindowType) {
        match wtype {
            WindowType::Footer => self.render_footer(),
            _ => (),
        }
    }

    const color_table: phf::Map<char, Color> = phf::phf_map! {
        '0' => Color::Reset,
        'a' => Color::Red,
        'b' => Color::Magenta,
        'c' => Color::Yellow,
        'd' => Color::Blue,
        'e' => Color::White,
        'f' => Color::Black,
    };

    pub fn render_footer(&mut self) {
        let color = [
            "bffffffffbbbbbbbbbbf",
            "fbbbbbbbbffffffffff0",
            "faaaaaaaafffffa",
            "0ffffffffaaaaaf",
        ];
        let line = String::from(" %mode  %name %space %lang  %pos ");
        let line = line.replace("%mode", self.keycontroller.mode_name());
        let line = line.replace(
            "%pos",
            format!("{}:{}", self.cursor.y, self.cursor.x).as_str(),
        );
        let line = line.replace("%name", &self.render.windows[1].name);

        let line = line.replace("%space", &{
            let line_len = line.graphemes(true).count() - 6;
            let win_len = self.render.windows[2].bottom_right.x - self.render.windows[2].top_left.x;
            " ".repeat(win_len as usize - line_len)
        });
        let index = self.render.windows[2].top_left.y as usize;

        self.buffer[index] = line
            .graphemes(true)
            .map(|c| Sign::new(c.to_string()))
            .collect();
        Self::color_line(&mut self.buffer[index], color);
        self.update_lines.push(index);
    }
    fn color_line(v: &mut Vec<Sign>, color_table: [&'static str; 4]) {
        for (i, c) in color_table[0].chars().enumerate() {
            if let Some(color) = Self::color_table.get(&c) {
                v[i].fg_color = *color;
            }
        }
        for (i, c) in color_table[1].chars().enumerate() {
            if let Some(color) = Self::color_table.get(&c) {
                v[i].bg_color = *color;
            }
        }
        for (i, c) in color_table[2].chars().rev().enumerate() {
            let len = v.len() - 1;
            if let Some(color) = Self::color_table.get(&c) {
                v[len - i].fg_color = *color;
            }
        }
        for (i, c) in color_table[3].chars().rev().enumerate() {
            let len = v.len() - 1;
            if let Some(color) = Self::color_table.get(&c) {
                v[len - i].bg_color = *color;
            }
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
