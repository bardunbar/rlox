

pub struct Token {

}

pub struct Scanner {

    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source: source, tokens: Vec::new() }
    }

    pub fn scan_tokens(&mut self) {
        println!("{}", self.source)
    }
}