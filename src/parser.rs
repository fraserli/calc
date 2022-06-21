// <expr>       ::= <NUMBER> <expr_opt>
// <expr_opt>   ::= + <NUMBER> <expr_opt> | epsilon

use crate::lexer::{Token, TokenType};

use anyhow::{bail, Context, Result};

#[derive(Debug)]
pub struct Expr {
    pub number: f64,
    pub opt: ExprOpt,
}

#[derive(Debug)]
pub enum ExprOpt {
    ExprOpt(f64, Box<ExprOpt>),
    None,
}

pub fn parse(mut tokens: Vec<Token>) -> Result<Expr> {
    let expr = expr(&mut tokens)?;

    if !tokens.is_empty() {
        bail!(
            "trailing characters: '{}'",
            tokens
                .iter()
                .map(|t| t.value.to_owned())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    Ok(expr)
}

fn eat<'a>(tokens: &mut Vec<Token<'a>>, ttype: TokenType) -> Result<Token<'a>> {
    if let Some(token) = tokens.get(0) {
        if token.ttype != ttype {
            bail!(
                "expected {:?} at position {}, got {:?} '{}'",
                ttype,
                token.pos,
                token.ttype,
                token.value
            );
        }
        Ok(tokens.remove(0))
    } else {
        bail!("unexpected EOF, expected {:?}", ttype);
    }
}

fn expr(tokens: &mut Vec<Token>) -> Result<Expr> {
    let num_tok = eat(tokens, TokenType::Number)?;
    let opt = expr_opt(tokens)?;

    let number = num_tok
        .value
        .parse()
        .with_context(|| format!("invalid number literal: {}", num_tok.value))?;

    Ok(Expr { number, opt })
}

fn expr_opt(tokens: &mut Vec<Token>) -> Result<ExprOpt> {
    if let Some(token) = tokens.get(0) {
        if token.ttype == TokenType::Addition {
            eat(tokens, TokenType::Addition)?;
            let num_tok = eat(tokens, TokenType::Number)?;
            let number = num_tok
                .value
                .parse()
                .with_context(|| format!("invalid number literal: {}", num_tok.value))?;
            return Ok(ExprOpt::ExprOpt(number, Box::new(expr_opt(tokens)?)));
        }
    }

    Ok(ExprOpt::None)
}
