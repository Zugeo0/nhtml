use super::source::Source;

// Position represents a selection in the source code.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    pub idx: usize,
    pub len: usize,
    pub start_ln: usize,
    pub start_cn: usize,
    pub end_ln: usize,
    pub end_cn: usize,
}

impl Position {
    pub fn new() -> Self {
        Self::default()
    }

    // Extends the range of the position to the next character
    // taking into account newlines
    pub fn extend(&mut self, source: &str) {
        let char = source.get_char(&self);
        let newline = matches!(char, Some('\n'));

        self.len += 1;
        self.end_cn += 1;

        if newline {
            self.end_cn = 1;
            self.end_ln += 1;
        }
    }

    // Moves the selection to the next character after the current selection
    pub fn advance(&mut self, source: &str) {
        self.extend(source);
        self.idx += self.len - 1;
        self.start_cn = self.end_cn;
        self.start_ln = self.end_ln;
        self.len = 1;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            idx: 0,
            len: 1,
            start_ln: 1,
            start_cn: 1,
            end_ln: 1,
            end_cn: 2,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.start_ln, self.start_cn))
    }
}
