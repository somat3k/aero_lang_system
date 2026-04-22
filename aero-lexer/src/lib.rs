use logos::Logos;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexError {
    #[error("Invalid token at position {0}")]
    InvalidToken(usize),
    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
pub enum Token {
    // Keywords
    #[token("fn")]
    Fn,
    #[token("know")]
    Know,
    #[token("emit")]
    Emit,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("world")]
    World,
    #[token("use")]
    Use,
    #[token("pub")]
    Pub,
    #[token("mod")]
    Mod,
    #[token("match")]
    Match,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Identifiers and literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    String(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Integer(i64),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    // Symbols
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token("=")]
    Assign,
    #[token("!")]
    Bang,
    #[token("?")]
    Question,
    #[token(".")]
    Dot,
    #[token("->")]
    Arrow,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,

    // Attributes
    #[token("#")]
    Hash,

    // End of file
    Eof,
}

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(source);

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(token) => tokens.push(token),
            Err(_) => {
                return Err(LexError::InvalidToken(lexer.span().start));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple_function() {
        let source = r#"
            fn main() {
                know x = 42;
            }
        "#;

        let tokens = lex(source).unwrap();
        assert!(tokens.contains(&Token::Fn));
        assert!(tokens.contains(&Token::Know));
    }

    #[test]
    fn test_lex_effects() {
        let source = r#"fn test() ! [log, http] {}"#;
        let tokens = lex(source).unwrap();
        assert!(tokens.contains(&Token::Fn));
        assert!(tokens.contains(&Token::Bang));
        assert!(tokens.contains(&Token::LBracket));
        assert!(tokens.contains(&Token::RBracket));
    }

    #[test]
    fn test_lex_string_literal() {
        let source = r#""Hello, AERO!""#;
        let tokens = lex(source).unwrap();
        assert!(matches!(tokens[0], Token::String(ref s) if s == "Hello, AERO!"));
    }

    #[test]
    fn test_lex_numbers() {
        let source = "42 3.14";
        let tokens = lex(source).unwrap();
        assert!(matches!(tokens[0], Token::Integer(42)));
        assert!(matches!(tokens[1], Token::Float(f) if (f - 3.14).abs() < 0.001));
    }
}
