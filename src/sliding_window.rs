pub struct SlidingWindow {
    target: Vec<char>,
    index: usize,
}

impl SlidingWindow {
    pub fn new(target_str: &str) -> SlidingWindow {
        SlidingWindow {
            target: target_str.chars().collect(),
            index: 1,
        }
    }

    pub fn slide(&mut self, glyph: char) -> bool {
        if self.target.len() < self.index {
            self.index = 0;
        }

        if glyph != self.target[self.index - 1] {
            self.index = 0;
        }

        self.index += 1;
        self.index > self.target.len()
    }
}
