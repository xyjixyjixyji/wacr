#![allow(clippy::upper_case_acronyms)]
use enum_as_inner::EnumAsInner;
use logos::{Lexer, Logos};
use std::{fmt, num::ParseIntError};
use strum_macros::AsRefStr;

fn from_hex<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<i64, String> {
    let slice = lex.slice();
    // Skip over the 0x in a hex number
    let without_prefix = &slice[2..slice.len()];

    // For Rust reasons we parse as i64 and then cast
    let res = u64::from_str_radix(without_prefix, 16);
    if let Ok(x) = res {
        if x <= 0xffffffff {
            Ok(x as i64)
        } else {
            Err(format!("{} is > 0xffffffff", without_prefix))
        }
    } else {
        Err(format!(
            "Failed Conversion with Error {:?}",
            res.unwrap_err()
        ))
    }
}

fn from_num<'b>(lex: &mut Lexer<'b, Token<'b>>) -> Result<i64, String> {
    let slice = lex.slice();

    let res: Result<i64, ParseIntError> = slice.parse();

    if res.is_err() {
        return Err(format!("Parsing failed wtih Error {:?}", res.unwrap_err()));
    }
    let out = res.unwrap();
    if out > ((i32::MIN as i64).abs()) {
        return Err(format!("Number {} is out of bounds", out));
    }
    Ok(out)
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[derive(Clone, Logos, Debug, PartialEq, AsRefStr, EnumAsInner)]
#[logos(subpattern identifier = r"[A-Za-z_][A-Za-z0-9_]*")]
pub enum Token<'a> {
    #[regex(r"(?&identifier)")]
    Ident(&'a str),

    #[regex(r"0|[1-9][0-9]*", from_num)]
    #[regex(r"0[xX][0-9a-fA-F]+", from_hex)]
    Number(i64),

    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Div,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
    #[token("~")]
    BitNot,
    #[token("=")]
    Assgn,
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("*=")]
    TimesEq,
    #[token("/=")]
    DivEq,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,

    // Reserved Keywords
    #[token("return")]
    Return,
    #[token("continue")]
    Continue,
    #[token("break")]
    Break,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("bool")]
    Bool,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("int")]
    Int,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[error]
    #[token("char")]
    #[token("string")]
    #[regex(r"[ \t\n\v\r\f]", logos::skip)] // Whitespace
    #[regex(r#"[^\x00-\x7F]"#)] // Error on non ascii characters
    Error,

    #[regex(r#"(//)[^\n]*"#, logos::skip)]
    Comment, // single line
}
