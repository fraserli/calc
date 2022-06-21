use crate::parser::{Expr, ExprOpt};

impl Expr {
    pub fn eval(&self) -> f64 {
        self.number + self.opt.eval()
    }
}

impl ExprOpt {
    pub fn eval(&self) -> f64 {
        if let ExprOpt::ExprOpt(num, opt) = self {
            num + opt.eval()
        } else {
            0.0
        }
    }
}
