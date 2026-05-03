use super::parser::{Expr, Operation};

pub fn evaluate(expr: &Expr) -> Option<f64> {
    match expr {
        Expr::Number(n) => Some(*n),
        Expr::BinaryOp { op, left, right } => {
            let l = evaluate(left)?;
            let r = evaluate(right)?;
            match op {
                Operation::ADD => Some(l + r),
                Operation::SUB => Some(l - r),
                Operation::MUL => Some(l * r),
                Operation::DIV => {
                    if r == 0.0 { return None; }
                    Some(l / r)
                }
            }
        }
    }
}