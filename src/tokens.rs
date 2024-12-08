const LOOKAHEAD: usize = 3;

enum TokenType {
    EXCLAMATION,
    UNDERSCORE,
    ASTERISK,
    GREATER,
    SPACE,
    HASH,
    DASH,
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    BACKTICK,
    TRIPLETICK,
    LINE,
    EOF,
    STR,
    EQ,
    ENUM,
}

pub struct Token {
    token_type: TokenType,
    value: Option<String>,
}

impl Token {
    fn new<S: AsRef<str>>(token_type: TokenType, value: Option<S>) -> Token {
        let value = value.map(|s| s.as_ref().to_string());
        Token { token_type, value }
    }
}

pub fn tokenize<S: AsRef<str>>(input: S) -> Vec<Token> {
    todo!();
}