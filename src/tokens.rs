#[derive(PartialEq, Debug)]
enum TokenType {
    CHAR,
	HASH,
	STAR,
	DASH,
	BANG,
    DIGIT,
    SPACE,
    PERIOD,
    RPAREN,
	LPAREN,
    NEWLINE,
    GREATER,
    RBRACKET,
	LBRACKET,
    BACKTICK,
    UNDERSCORE,
}

impl TokenType {
    fn from_char(c: char) -> TokenType {
        match c {
            '#' => TokenType::HASH,
            '*' => TokenType::STAR,
            '-' => TokenType::DASH,
            '!' => TokenType::BANG,
            '0'..='9' => TokenType::DIGIT,
            ' ' => TokenType::SPACE,
            '.' => TokenType::PERIOD,
            '(' => TokenType::LPAREN,
            ')' => TokenType::RPAREN,
            '\n' => TokenType::NEWLINE,
            '>' => TokenType::GREATER,
            '[' => TokenType::LBRACKET,
            ']' => TokenType::RBRACKET,
            '`' => TokenType::BACKTICK,
            '_' => TokenType::UNDERSCORE,
            _ => TokenType::CHAR,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Token {
    line: usize,
    value: char,
    kind: TokenType,
    position: usize,
}

impl Token {
    fn new(value: char, position: usize, line: usize) -> Token {
        let kind = TokenType::from_char(value);
        Token { value, kind, position, line }
    }
}

pub fn tokenize<S: AsRef<str>>(input: S) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line = 0;
    let mut position = 0;
    for c in input.as_ref().chars() {
        tokens.push(Token::new(c, position, line));
        if c == '\n' {
            line += 1;
            position = 0;
            continue;
        }
        position += 1;
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "## Hi!,\n ";
        let expected = vec![
            Token::new('#', 0, 0),
            Token::new('#', 1, 0),
            Token::new(' ', 2, 0),
            Token::new('H', 3, 0),
            Token::new('i', 4, 0),
            Token::new('!', 5, 0),
            Token::new(',', 6, 0),
            Token::new('\n', 7, 0),
            Token::new(' ', 0, 1),
        ];
        assert_eq!(tokenize(input), expected);
    }
}