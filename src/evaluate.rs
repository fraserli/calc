use crate::parser::{Expr, ExprOpt, Term, TermOpt};

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
        self.number * self.opt.eval()
    }
}

impl TermOpt {
    pub fn eval(&self) -> f64 {
        if let TermOpt::TermOpt(num, opt) = self {
            num * opt.eval()
        } else {
            1.0
        }
    }
}
