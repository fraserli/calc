use crate::parser::{Expr, ExprOpt, Factor, Term, TermOpt};

impl Expr {
    pub fn eval(&self) -> f64 {
        self.term.eval() + self.opt.eval()
    }
}

impl ExprOpt {
    pub fn eval(&self) -> f64 {
        if let ExprOpt::ExprOpt(term, opt) = self {
            term.eval() + opt.eval()
        } else {
            0.0
        }
    }
}

impl Term {
    pub fn eval(&self) -> f64 {
        self.factor.eval() * self.opt.eval()
    }
}

impl TermOpt {
    pub fn eval(&self) -> f64 {
        if let TermOpt::TermOpt(factor, opt) = self {
            factor.eval() * opt.eval()
        } else {
            1.0
        }
    }
}

impl Factor {
    pub fn eval(&self) -> f64 {
        match self {
            Factor::Number(number) => *number,
            Factor::Expr(expr) => expr.eval(),
        }
    }
}
