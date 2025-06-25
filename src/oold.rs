struct Buffer {
    corners: (IVec2, IVec2),
    max_index_length: usize,
    clear_line: String,
    lines: Vec<String>,
}

impl Buffer {
    pub fn new(path: &String, corners: (IVec2, IVec2)) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let lines: Vec<String> = content.lines().map(String::from).collect();
        let max_index_length = (lines.len() as f64).log10().floor() as usize + 1;
        let clear_line = " ".repeat((corners.1.y - corners.0.y) as usize);
        Ok(Self {
            corners,
            max_index_length,
            clear_line,
            lines,
        })
    }
    pub fn resize(&mut self, w: u16, h: u16) {
        self.corners.1 = IVec2::new(w as i32, h as i32 - 2);
    }
}
fn all_buffer(&self) -> UIE {
    UIE::Buffer(
        0,
        self.scroll as usize + 1..self.scroll as usize + self.size.1 as usize - 1,
    )
}

enum UIE {
    Buffer(usize, Range<usize>), // 0: Bufferindex; 1: Bufferline
    Header,
    Footer,
}
