use glam::UVec2;
use unicode_segmentation::UnicodeSegmentation;

use crate::app::{App, Sign};

pub enum WindowType {
    Header,
    Text,
    Footer,
}

#[derive(Debug)]
pub struct Window {
    pub name: String,
    pub top_left: UVec2,
    pub bottom_right: UVec2,
}

impl Window {
    pub fn new(name: String, top_left: UVec2, bottom_right: UVec2) -> Self {
        Self {
            name,
            top_left,
            bottom_right,
        }
    }
}

#[derive(Debug)]
pub struct Render {
    pub windows: Vec<Window>,
    register: Vec<Vec<usize>>,
    size: UVec2,
}

impl Render {
    pub fn new(size: (u16, u16)) -> Self {
        let size = UVec2::new(size.0 as u32, size.1 as u32);
        let windows = vec![
            Window::new("Header".into(), UVec2::new(0, 0), UVec2::new(size.x, 0)),
            Window::new(
                "render.rs".into(),
                UVec2::new(0, 1),
                UVec2::new(size.x, size.y - 2),
            ),
            Window::new(
                "Footer".into(),
                UVec2::new(0, size.y - 1),
                UVec2::new(size.x, size.y - 1),
            ),
        ];
        Self {
            windows,
            register: vec![],
            size,
        }
    }
}
