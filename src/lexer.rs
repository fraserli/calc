use anyhow::{bail, Result};

#[derive(Debug)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub value: &'a str,
    pub pos: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    Number,
    Addition,
    Multiplication,
}

pub fn lex(text: &str) -> Result<Vec<Token>> {
    use TokenType::*;

    let mut tokens = Vec::new();

    let mut iter = text.chars().enumerate().peekable();
    while let Some((i, char)) = iter.next() {
        if char.is_whitespace() {
            continue;
        } else if char == '+' {
            tokens.push(Token {
                ttype: Addition,
                value: &text[i..i + 1],
                pos: i,
            })
        } else if char == '*' {
            tokens.push(Token {
                ttype: Multiplication,
                value: &text[i..i + 1],
                pos: i,
            })
        } else if char.is_ascii_digit() {
            let start = i;
            let mut end = i + 1;
            while let Some((i, _)) = iter.next_if(|(_, c)| c.is_ascii_digit()) {
                end = i + 1;
            }
            tokens.push(Token {
                ttype: Number,
                value: &text[start..end],
                pos: start,
            });
        } else if char.is_ascii() {
            let start = i;
            let mut end = i + 1;
            while let Some((i, _)) = iter.next_if(|(_, c)| !c.is_whitespace()) {
                end = i + 1;
            }
            tokens.push(Token {
                ttype: Identifier,
                value: &text[start..end],
                pos: start,
            });
        } else {
            bail!("invalid character: '{char}'");
        }
    }

    Ok(tokens)
}
