pub struct SourceController {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl SourceController {
    pub fn new(source: &str) -> Self {
        SourceController {
            source: String::from(source),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn set_start_to_next_token(&mut self) {
        self.start = self.current
    }

    pub fn query_current(&self) -> char {
        if self.is_eof() {
            return '\0';
        }
        self.query(self.current)
    }

    pub fn query_previous(&self) -> char {
        self.query(self.current - 1)
    }

    pub fn query_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.query(self.current + 1)
    }

    pub fn skip_whitespaces(&mut self) {
        loop {
            let char = self.query_current();

            match char {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.query_next() == '/' {
                        while self.query_current() != '\n' && !self.is_eof() {
                            self.advance();
                        }
                    }
                }
                _ => return (),
            };
        }
    }

    fn query(&self, n: usize) -> char {
        self.source.chars().nth(n).unwrap()
    }

    pub fn advance(&mut self) -> char {
        let current = self.query_current();
        self.current += 1;
        current
    }

    pub fn advance_match(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        }

        if self.query_current() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    pub fn extract_from_start(&self) -> String {
        self.extract_as_string(self.start, self.current)
    }

    pub fn extract_as_string(&self, start: usize, end: usize) -> String {
        String::from(&self.source.as_str()[start..end])
    }

    pub fn is_eof(&self) -> bool {
        self.current as usize >= self.source.len()
    }

    pub fn is_closed(&self, enclosing: char) -> bool {
        self.query_current() == enclosing || self.is_eof()
    }

    pub fn is_newline(&self) -> bool {
        self.query_current() == '\n'
    }
}
