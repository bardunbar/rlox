

static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
    "and" =>TokenType::And,
    "class" =>TokenType::Class,
    "else" =>TokenType::Else,
    "false" =>TokenType::False,
    "for" =>TokenType::For,
    "fun" =>TokenType::Fun,
    "if" =>TokenType::If,
    "nil" =>TokenType::Nil,
    "or" =>TokenType::Or,
    "print" =>TokenType::Print,
    "return" =>TokenType::Return,
    "super" =>TokenType::Super,
    "this" =>TokenType::This,
    "true" =>TokenType::True,
    "var" =>TokenType::Var,
    "while" =>TokenType::While,
};

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

#[derive(Debug, Clone, Copy)]
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
    Print, Return, Super, This, True, Var, While,

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

#[derive(Debug)]
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
    chars: Vec<char>,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars: Vec<_> = source.chars().collect();
        Scanner { source: source, chars: chars, tokens: Vec::new() , start: 0, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self, environment: &mut Environment) {

        while self.current < self.chars.len()
        {
            self.start = self.current;
            self.scan_token(environment);
        }

        self.tokens.push(Token::new(TokenType::EOF, String::new(), Literal::None, self.line))
    }

    fn scan_token(&mut self, environment: &mut Environment) -> bool {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::None),
            ')' => self.add_token(TokenType::RightParen, Literal::None),
            '{' => self.add_token(TokenType::LeftBrace, Literal::None),
            '}' => self.add_token(TokenType::RightBrace, Literal::None),
            ',' => self.add_token(TokenType::Comma, Literal::None),
            '.' => self.add_token(TokenType::Dot, Literal::None),
            '-' => self.add_token(TokenType::Minus, Literal::None),
            '+' => self.add_token(TokenType::Plus, Literal::None),
            ';' => self.add_token(TokenType::Semicolon, Literal::None),
            '*' => self.add_token(TokenType::Star, Literal::None),

            '!' => {
                let token_type = if self.match_char('=') {TokenType::BangEqual} else {TokenType::Bang};
                self.add_token(token_type, Literal::None)
            },
            '=' => {
                let token_type = if self.match_char('=') {TokenType::EqualEqual} else {TokenType::Equal};
                self.add_token(token_type, Literal::None)
            },
            '<' => {
                let token_type = if self.match_char('=') {TokenType::LessEqual} else {TokenType::Less};
                self.add_token(token_type, Literal::None)
            },
            '>' => {
                let token_type = if self.match_char('=') {TokenType::GreaterEqual} else {TokenType::Greater};
                self.add_token(token_type, Literal::None)
            },

            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                } else {
                    self.add_token(TokenType::Slash, Literal::None);
                }
            }

            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => self.line = self.line + 1,

            '"' => self.extract_string(environment),

            '0'..='9' => self.extract_number(environment),

            'a'..='z' | 'A'..='Z' | '_' => self.extract_identifier(environment),

            _ => {
                environment.error(self.line, "Unexpected character.");
            }
        }

        true
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current].to_owned();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn advance(&mut self) -> char {
        let result = self.chars[self.current];
        self.current = self.current + 1;
        result
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }

    fn extract_string(&mut self, environment: &mut Environment) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line = self.line + 1 }
            self.advance();
        }

        if self.is_at_end() {
            environment.error(self.line, "Unterminated string.");
            return
        }

        self.advance();

        // Trim the " from the string
        let literal = self.source[self.start + 1..self.current - 1].to_owned();

        self.add_token(TokenType::String, Literal::String(literal))
    }

    fn extract_number(&mut self, environment: &mut Environment) {
        while Scanner::is_digit(&self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(&self.peek_next()) {
            self.advance();

            while Scanner::is_digit(&self.peek()) {
                self.advance();
            }
        }

        let literal = &self.source[self.start..self.current];
        let literal_number: f64 = match literal.parse() {
            Ok(num) => num,
            Err(_) => {
                environment.error(self.line, "Unable to parse number");
                0.0
            }
        };

        self.add_token(TokenType::Number, Literal::Number(literal_number))
    }

    fn extract_identifier(&mut self, environment: &mut Environment) {
        while Scanner::is_alphanumeric(&self.peek()) {
            self.advance();
        }

        let literal_string = &self.source[self.start..self.current];
        let (token_type, literal) = match KEYWORDS.get(literal_string) {
            Some(t) => ((*t).clone(), Literal::None),
            None => (TokenType::Identifier, Literal::Identifier(literal_string.to_owned())),
        };

        self.add_token(token_type, literal)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.chars[self.current] != expected {
            false
        } else {
            self.current = self.current + 1;
            true
        }
    }

    pub fn debug_print_tokens(&self) {
        for token in self.tokens.iter() {
            println!("{:?}", token)
        }
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    #[inline]
    fn is_digit(c: &char) -> bool {
        *c >= '0' && *c <= '9'
    }

    #[inline]
    fn is_alpha(c: &char) -> bool {
        *c >= 'a' && *c <= 'z' || *c >= 'A' && *c <= 'Z' || *c == '_'
    }

    #[inline]
    fn is_alphanumeric(c: &char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }
}