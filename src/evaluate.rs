use crate::parser::{Expr, ExprOperation, Factor, Term, TermOpt};

impl Expr {
    pub fn eval(&self) -> f64 {
        let mut total = self.term.eval();

        for (op, term) in self.opts.iter() {
            match op {
                ExprOperation::Addition => total += term.eval(),
                ExprOperation::Subtraction => total -= term.eval(),
            }
        }

        total
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
