use std::str::FromStr;

use crate::utils::SourcePosition;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_kind: TokenKind,
    pub spelling: String,
    pub token_position: SourcePosition,
}

impl Token {
    pub fn new(kind: TokenKind, spelling: String, position: SourcePosition) -> Self {
        if kind == TokenKind::ID {
            let kind = TokenKind::from_spelling(&spelling);
            Self {
                token_kind: kind,
                spelling,
                token_position: position,
            }
        } else {
            Self {
                token_kind: kind,
                spelling,
                token_position: position,
            }
        }
    }

    pub fn kind_to_string(&self) -> &str {
        self.token_kind.as_str()
    }
    // Use default implementation of ToString derived from Debug
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // Token kinds referred to by name.  As per enum defintion.
    BOOLEAN,
    BREAK,
    CONTINUE,
    ELSE,
    FLOAT,
    FOR,
    IF,
    INT,
    RETURN,
    VOID,
    WHILE,

    // operators
    PLUS,
    MINUS,
    MULT,
    DIV,
    NOT,
    NOTEQ,
    EQ,
    EQEQ,
    LT,
    LTEQ,
    GT,
    GTEQ,
    ANDAND,
    OROR,

    // separators
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    SEMICOLON,
    COMMA,

    // identifiers
    ID,

    // literals
    INTLITERAL,
    FLOATLITERAL,
    BOOLEANLITERAL,
    STRINGLITERAL,

    // special tokens
    ERROR,
    EOF,
}

impl TokenKind {
    pub fn from_spelling(spelling: &str) -> Self {
        // Your implementation here
        // This function should return a TokenKind based on the spelling
        // In this stub, we're returning a default
        TokenKind::ERROR
    }

    pub fn as_str(&self) -> &str {
        "TODO"
    }
}

impl FromStr for TokenKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "boolean" => Ok(TokenKind::BOOLEAN),
            "break" => Ok(TokenKind::BREAK),
            "continue" => Ok(TokenKind::CONTINUE),
            "else" => Ok(TokenKind::ELSE),
            "float" => Ok(TokenKind::FLOAT),
            "for" => Ok(TokenKind::FOR),
            "if" => Ok(TokenKind::IF),
            "int" => Ok(TokenKind::INT),
            "return" => Ok(TokenKind::RETURN),
            "void" => Ok(TokenKind::VOID),
            "while" => Ok(TokenKind::WHILE),
            "+" => Ok(TokenKind::PLUS),
            "-" => Ok(TokenKind::MINUS),
            "*" => Ok(TokenKind::MULT),
            "/" => Ok(TokenKind::DIV),
            "!" => Ok(TokenKind::NOT),
            "!=" => Ok(TokenKind::NOTEQ),
            "=" => Ok(TokenKind::EQ),
            "==" => Ok(TokenKind::EQEQ),
            "<" => Ok(TokenKind::LT),
            "<=" => Ok(TokenKind::LTEQ),
            ">" => Ok(TokenKind::GT),
            ">=" => Ok(TokenKind::GTEQ),
            "&&" => Ok(TokenKind::ANDAND),
            "||" => Ok(TokenKind::OROR),
            "{" => Ok(TokenKind::LBRACE),
            "}" => Ok(TokenKind::RBRACE),
            "(" => Ok(TokenKind::LPAREN),
            ")" => Ok(TokenKind::RPAREN),
            "[" => Ok(TokenKind::LBRACKET),
            "]" => Ok(TokenKind::RBRACKET),
            ";" => Ok(TokenKind::SEMICOLON),
            "," => Ok(TokenKind::COMMA),
            "<int-literal>" => Ok(TokenKind::INTLITERAL),
            "<float-literal>" => Ok(TokenKind::FLOATLITERAL),
            "<boolean-literal>" => Ok(TokenKind::BOOLEANLITERAL),
            "<string-literal>" => Ok(TokenKind::STRINGLITERAL),
            "<error>" => Ok(TokenKind::ERROR),
            "$" => Ok(TokenKind::EOF),
            _ => Ok(TokenKind::ID),
        }
    }
}

impl std::string::ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::BOOLEAN => String::from("boolean"),
            TokenKind::BREAK => String::from("break"),
            TokenKind::CONTINUE => String::from("continue"),
            TokenKind::ELSE => String::from("else"),
            TokenKind::FLOAT => String::from("float"),
            TokenKind::FOR => String::from("for"),
            TokenKind::IF => String::from("if"),
            TokenKind::INT => String::from("int"),
            TokenKind::RETURN => String::from("return"),
            TokenKind::VOID => String::from("void"),
            TokenKind::WHILE => String::from("while"),
            TokenKind::PLUS => String::from("+"),
            TokenKind::MINUS => String::from("-"),
            TokenKind::MULT => String::from("*"),
            TokenKind::DIV => String::from("/"),
            TokenKind::NOT => String::from("!"),
            TokenKind::NOTEQ => String::from("!="),
            TokenKind::EQ => String::from("="),
            TokenKind::EQEQ => String::from("=="),
            TokenKind::LT => String::from("<"),
            TokenKind::LTEQ => String::from("<="),
            TokenKind::GT => String::from(">"),
            TokenKind::GTEQ => String::from(">="),
            TokenKind::ANDAND => String::from("&&"),
            TokenKind::OROR => String::from("||"),
            TokenKind::LBRACE => String::from("{"),
            TokenKind::RBRACE => String::from("}"),
            TokenKind::LPAREN => String::from("("),
            TokenKind::RPAREN => String::from(")"),
            TokenKind::LBRACKET => String::from("["),
            TokenKind::RBRACKET => String::from("]"),
            TokenKind::SEMICOLON => String::from(";"),
            TokenKind::COMMA => String::from(","),
            TokenKind::ID => String::from("<id>"),
            TokenKind::INTLITERAL => String::from("<int-literal>"),
            TokenKind::FLOATLITERAL => String::from("<float-literal>"),
            TokenKind::BOOLEANLITERAL => String::from("<boolean-literal>"),
            TokenKind::STRINGLITERAL => String::from("<string-literal>"),
            TokenKind::ERROR => String::from("<error>"),
            TokenKind::EOF => String::from("$"),
        }
    }
}
