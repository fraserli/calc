// <expr>           ::= <term> <expr_opt>
// <expr_opt>       ::= <expr_operation> <term> <expr_opt>*
// <expr_operation> ::= '+' | '-'
// <term>           ::= <factor> <term_opt>
// <term_opt>       ::= '*' <factor> <term_opt> | epsilon
// <factor>         ::= '-'* <NUMBER> | '-'* (<expr>)

use crate::lexer::{Token, TokenType};

use anyhow::{bail, Context, Result};

use std::ops::Neg;

#[derive(Debug)]
pub struct Expr {
    pub term: Term,
    pub opts: Vec<(ExprOperation, Term)>,
}

#[derive(Debug)]
pub enum ExprOperation {
    Addition,
    Subtraction,
}

#[derive(Debug)]
pub struct Term {
    pub factor: Factor,
    pub opt: TermOpt,
}

#[derive(Debug)]
pub enum TermOpt {
    TermOpt(Factor, Box<TermOpt>),
    None,
}

#[derive(Debug)]
pub enum Factor {
    Number(f64),
    Expr(Box<Expr>),
}

impl Neg for Factor {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Self::Number(number) => Self::Number(-number),
            Self::Expr(e) => {
                let mut opts = e
                    .opts
                    .into_iter()
                    .map(|(op, term)| match op {
                        ExprOperation::Addition => (ExprOperation::Subtraction, term),
                        ExprOperation::Subtraction => (ExprOperation::Addition, term),
                    })
                    .collect::<Vec<(ExprOperation, Term)>>();
                opts.push((ExprOperation::Subtraction, e.term));
                let expr = Expr {
                    term: Term {
                        factor: Factor::Number(0.0),
                        opt: TermOpt::None,
                    },
                    opts,
                };
                Factor::Expr(Box::new(expr))
            }
        }
    }
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

fn try_eat<'a>(tokens: &mut Vec<Token<'a>>, ttype: TokenType) -> Result<Option<Token<'a>>> {
    if let Some(token) = tokens.get(0) {
        if token.ttype == ttype {
            Ok(Some(eat(tokens, ttype)?))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn expr(tokens: &mut Vec<Token>) -> Result<Expr> {
    let term = term(tokens)?;
    let opts = expr_opt(tokens)?;

    Ok(Expr { term, opts })
}

fn expr_opt(tokens: &mut Vec<Token>) -> Result<Vec<(ExprOperation, Term)>> {
    let mut opts = Vec::new();

    loop {
        if try_eat(tokens, TokenType::Addition)?.is_some() {
            opts.push((ExprOperation::Addition, term(tokens)?))
        } else if try_eat(tokens, TokenType::Subtraction)?.is_some() {
            opts.push((ExprOperation::Subtraction, term(tokens)?))
        } else {
            break;
        }
    }

    Ok(opts)
}

fn term(tokens: &mut Vec<Token>) -> Result<Term> {
    let factor = factor(tokens)?;
    let opt = term_opt(tokens)?;

    Ok(Term { factor, opt })
}

fn term_opt(tokens: &mut Vec<Token>) -> Result<TermOpt> {
    if try_eat(tokens, TokenType::Multiplication)?.is_some() {
        let factor = factor(tokens)?;
        Ok(TermOpt::TermOpt(factor, Box::new(term_opt(tokens)?)))
    } else {
        Ok(TermOpt::None)
    }
}

fn factor(tokens: &mut Vec<Token>) -> Result<Factor> {
    if try_eat(tokens, TokenType::Addition)?.is_some() {
        Ok(factor(tokens)?)
    } else if try_eat(tokens, TokenType::Subtraction)?.is_some() {
        Ok(-factor(tokens)?)
    } else if try_eat(tokens, TokenType::OpeningParen)?.is_some() {
        let expr = expr(tokens)?;
        eat(tokens, TokenType::ClosingParen)?;
        Ok(Factor::Expr(Box::new(expr)))
    } else {
        let number = number(tokens)?;
        Ok(Factor::Number(number))
    }
}

fn number(tokens: &mut Vec<Token>) -> Result<f64> {
    let num_tok = eat(tokens, TokenType::Number)?;
    let number = num_tok
        .value
        .parse()
        .with_context(|| format!("invalid number literal: {}", num_tok.value))?;
    Ok(number)
}
