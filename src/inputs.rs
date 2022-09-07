use tui::text::Text;

#[derive(Clone)]
pub struct Input {
    pub name: String,
    pub text: String,
    pub pos: usize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            text: String::new(),
            pos: 0,
        }
    }

    pub fn from(text: String) -> Self {
        Self {
            name: String::new(),
            text: text.clone(),
            pos: text.len(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn last(&self) -> String {
        if self.len() > 0 {
            self.text[self.len() - 1..].to_string()
        } else {
            String::new()
        }
    }

    pub fn num_trailing_spaces(&self) -> usize {
        let mut text = self.text.clone();
        let mut counter = 0;
        while text.len() > 0 && &text[text.len() - 1..] == " " {
            counter += 1;
            text.pop();
        }

        counter
    }

    pub fn push(&mut self, c: char) {
        // Leading spaces messes with input rendering, so we don't allow it :)
        if self.pos == 0 && c == ' ' {
            return;
        }

        self.text.insert(self.pos, c);
        self.pos += 1;
    }

    pub fn pop(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
            self.text.remove(self.pos);
        }
    }

    pub fn extract(&mut self) -> String {
        self.pos = 0;
        self.text.drain(..).collect()
    }

    pub fn clear(&mut self) {
        self.pos = 0;
        self.text.clear();
    }

    pub fn move_left(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.pos < self.len() {
            self.pos += 1;
        }
    }

    pub fn move_start(&mut self) {
        self.pos = 0;
    }

    pub fn move_end(&mut self) {
        self.pos = self.len();
    }

    pub fn move_to_prev_space(&mut self) {
        let prev_string = &self.text[..self.pos];
        let index = prev_string.rfind(' ');

        match index {
            Some(i) => self.pos = i,
            None => self.pos = 0
        }
    }

    pub fn move_to_next_space(&mut self) {
        let start = std::cmp::min(self.pos + 1, self.len());
        let next_string = &self.text[start..];
        let index = next_string.find(' ');

        match index {
            Some(i) => self.pos = self.pos + 1 + i,
            None => self.pos = self.len()
        }
    }

    pub fn delete_to_left_space(&mut self) {
        let prev_pos = self.pos;
        self.move_to_prev_space();
        let curr_pos = self.pos;

        let left = &self.text[..curr_pos];
        let right = &self.text[prev_pos..];

        self.text = format!("{}{}", left, right);
    }
}

impl<'a> From<Input> for Text<'a> {
    fn from(i: Input) -> Text<'a> {
        Text::raw(i.text)
    }
}

impl<'a> From<&Input> for Text<'a> {
    fn from(i: &Input) -> Text<'a> {
        Text::raw(i.clone().text)
    }
}

impl<'a> From<&mut Input> for Text<'a> {
    fn from(i: &mut Input) -> Text<'a> {
        Text::raw(i.clone().text)
    }
}
