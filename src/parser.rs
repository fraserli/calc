// <expr>       ::= <term> <expr_opt>
// <expr_opt>   ::= + <term> <expr_opt> | epsilon
// <term>       ::= <NUMBER> <term_opt>
// <term_opt>   ::= * <NUMBER> <term_opt> | epsilon

use crate::lexer::{Token, TokenType};

use anyhow::{bail, Context, Result};

#[derive(Debug)]
pub struct Expr {
    pub term: Term,
    pub opt: ExprOpt,
}

#[derive(Debug)]
pub enum ExprOpt {
    ExprOpt(Term, Box<ExprOpt>),
    None,
}

#[derive(Debug)]
pub struct Term {
    pub number: f64,
    pub opt: TermOpt,
}

#[derive(Debug)]
pub enum TermOpt {
    TermOpt(f64, Box<TermOpt>),
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
    let term = term(tokens)?;
    let opt = expr_opt(tokens)?;

    Ok(Expr { term, opt })
}

fn expr_opt(tokens: &mut Vec<Token>) -> Result<ExprOpt> {
    if let Some(token) = tokens.get(0) {
        if token.ttype == TokenType::Addition {
            eat(tokens, TokenType::Addition)?;
            let term = term(tokens)?;
            return Ok(ExprOpt::ExprOpt(term, Box::new(expr_opt(tokens)?)));
        }
    }

    Ok(ExprOpt::None)
}

fn term(tokens: &mut Vec<Token>) -> Result<Term> {
    let num_tok = eat(tokens, TokenType::Number)?;
    let opt = term_opt(tokens)?;

    let number = num_tok
        .value
        .parse()
        .with_context(|| format!("invalid number literal: {}", num_tok.value))?;

    Ok(Term { number, opt })
}

fn term_opt(tokens: &mut Vec<Token>) -> Result<TermOpt> {
    if let Some(token) = tokens.get(0) {
        if token.ttype == TokenType::Multiplication {
            eat(tokens, TokenType::Multiplication)?;
            let num_tok = eat(tokens, TokenType::Number)?;
            let number = num_tok
                .value
                .parse()
                .with_context(|| format!("invalid number literal: {}", num_tok.value))?;
            return Ok(TermOpt::TermOpt(number, Box::new(term_opt(tokens)?)));
        }
    }

    Ok(TermOpt::None)
}
