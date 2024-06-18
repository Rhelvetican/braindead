pub struct Lexer {
    src: String,
    filter: Vec<char>,
}

impl Lexer {
    pub fn new(src: String, filter: Vec<char>) -> Self {
        Self { src, filter }
    }
    pub fn lex(&self) -> Vec<String> {
        self.src
            .split(|c| self.filter.contains(&c))
            .map(|s| s.to_string())
            .collect()
    }
}
