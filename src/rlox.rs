



fn report(line: u32, source: &str, msg: &str) {
    eprintln!("[{}] Error {}: {}", line, source, msg)
}

pub struct Environment {
    had_error: bool
}

impl Environment {
    pub fn new() -> Self {
        Environment { had_error: false }
    }

    pub fn error(&mut self, line: u32, msg: &str) {
        report(line, "", msg);
        self.had_error = true;
    }

    #[inline]
    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn get_exit_code(&self) -> i32 {
        if self.had_error {
            65
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    // Single Character Tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or Two Character Tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, This, True, Var, While,

    EOF,
}

#[derive(Debug)]
struct Object;

#[derive(Debug)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
    None
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u32,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Literal, line: u32) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        String::from(format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal))
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source: source, tokens: Vec::new() , start: 0, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self, environment: &Environment) {

        let chars: Vec<_> = self.source.chars().collect();
        let length = chars.len();

        let mut characters = self.source.chars();

        while self.current < length
        {
            self.start = self.current;
            self.scan_token(&chars);
        }

        self.tokens.push(Token::new(TokenType::EOF, String::new(), Literal::None, self.line))
    }

    fn scan_token(&mut self, chars: &Vec<char>) -> bool {


        true
    }
}