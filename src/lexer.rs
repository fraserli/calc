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
    Subtraction,
    Multiplication,
    OpeningParen,
    ClosingParen,
}

pub fn lex(text: &str) -> Result<Vec<Token>> {
    use TokenType::*;

    let mut tokens = Vec::new();

    let mut iter = text.chars().enumerate().peekable();
    while let Some((i, char)) = iter.next() {
        if char.is_whitespace() {
            continue;
        }

        let (ttype, value) = match char {
            '+' => (Addition, &text[i..i + 1]),
            '-' => (Subtraction, &text[i..i + 1]),
            '*' => (Multiplication, &text[i..i + 1]),
            '(' => (OpeningParen, &text[i..i + 1]),
            ')' => (ClosingParen, &text[i..i + 1]),
            _ => {
                if char.is_ascii_digit() {
                    let start = i;
                    let mut end = i + 1;
                    while let Some((i, _)) = iter.next_if(|(_, c)| c.is_ascii_digit()) {
                        end = i + 1;
                    }
                    (Number, &text[start..end])
                } else if char.is_ascii() {
                    let start = i;
                    let mut end = i + 1;
                    while let Some((i, _)) = iter.next_if(|(_, c)| !c.is_whitespace()) {
                        end = i + 1;
                    }
                    (Identifier, &text[start..end])
                } else {
                    bail!("invalid character: '{char}'");
                }
            }
        };

        tokens.push(Token {
            ttype,
            value,
            pos: i,
        });
    }

    Ok(tokens)
}
