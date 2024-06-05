pub struct SlidingWindow {
    target: Vec<char>,
    index: usize,
}

impl SlidingWindow {
    pub fn new(target_str: &str) -> SlidingWindow {
        let mut glyphs = Vec::from(['<', '/']);
        glyphs.append(&mut target_str.chars().collect());
        SlidingWindow {
            target: glyphs,
            index: 1,
        }
    }

    pub fn slide(&mut self, glyph: char) -> bool {
        if self.index > self.target.len() {
            self.index = 0;
        }

        if self.target[self.index - 1] != glyph {
            self.index = 0;
        }

        self.index += 1;
        self.index > self.target.len()
    }
}
