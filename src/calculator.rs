use crate::math::Math;
use crate::parser::{evaluator, parser, tokenizer};

pub struct Calculator;

impl Calculator {
    pub fn calc(input: &str) -> Option<f64> {
        let tokens = tokenizer::tokenize(input)?;
        let mut p = parser::Parser::new(tokens);
        let ast = p.parse()?;
        evaluator::evaluate(&ast)
    }

    pub fn dec_to_frac<T: Into<f64>>(dec: T) -> String {

        let x = dec.into();

        if x.is_nan() { return "NaN".to_string(); }
        if x.is_infinite() { return if x > 0.0 { "∞".to_string() } else { "-∞".to_string() }; }

        let sign = if x < 0.0 { "-" } else { "" };
        let x = x.abs();
        let decimal = x.fract();

        if decimal == 0.0 { return format!("{}{}", sign, x.trunc() as i64); }

        let precision = 1_000_000_000_000_000_000_i64;
        let numerator = (x * precision as f64).round() as i64;
        let denominator = precision;
        let gcd =  Math::gcd(numerator.abs(), denominator);

        let n = numerator / gcd;
        let d = denominator / gcd;
        if d == 1 { return format!("{}{}", sign, n); }
        format!("{}{}/{}", sign, n, d)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

        // println!("{}", Math::add(12, 21).to_string()); // 33

        println!("{}", Calculator::calc("3+2").unwrap())
    }
}