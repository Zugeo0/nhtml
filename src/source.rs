use super::position::Position;

pub trait Source {
    // Get string slice at the specified source
    fn get_str<'a>(&'a self, pos: &Position) -> &'a str;

    // Get character at the end of the position slice
    fn get_char(&self, pos: &Position) -> Option<char>;
    
    // Get character at the start of the position slice
    fn get_char_start(&self, pos: &Position) -> Option<char>;

    // Gets the character right after the position
    fn peek_next(&self, pos: &Position) -> Option<char>;

    // Get the lines the position covers
    fn get_lines(&self, pos: &Position) -> Vec<String>;
}

impl Source for &str {
    fn get_str<'a>(&'a self, pos: &Position) -> &'a str {
        &self[pos.idx..pos.idx + pos.len]
    }

    fn get_char(&self, pos: &Position) -> Option<char> {
        self.chars().nth(pos.idx + pos.len - 1)
    }

    fn get_char_start(&self, pos: &Position) -> Option<char> {
        self.chars().nth(pos.idx)
    }

    fn peek_next(&self, pos: &Position) -> Option<char> {
        self.chars().nth(pos.idx + pos.len)
    }

    fn get_lines(&self, pos: &Position) -> Vec<String> {
        if pos.idx >= self.len() {
            return vec![];
        }

        self.lines()
            .skip(pos.start_ln - 1)
            .take(pos.end_ln - pos.start_ln + 1)
            .map(String::from)
            .collect()
    }
}
