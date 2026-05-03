use crate::parser::tokenizer::TOKEN;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    BinaryOp { op: Operation, left: Box<Expr>, right: Box<Expr> },
}

#[derive(Debug)]
pub enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
}

pub struct Parser {
    tokens: Vec<TOKEN>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TOKEN>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&TOKEN> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<&TOKEN> {
        let t = self.tokens.get(self.pos);
        self.pos += 1;
        t
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Option<Expr> {
        let mut left = self.parse_multiplicative()?;

        while let Some(op) = self.peek() {
            let op = match op {
                TOKEN::PLUS => Operation::ADD,
                TOKEN::MINUS => Operation::SUB,
                _ => break,
            };
            self.consume();
            let right = self.parse_multiplicative()?;
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }

        Some(left)
    }

    fn parse_multiplicative(&mut self) -> Option<Expr> {
        let mut left = self.parse_primary()?;

        while let Some(op) = self.peek() {
            let op = match op {
                TOKEN::MULTIPLE => Operation::MUL,
                TOKEN::DIVIDE => Operation::DIV,
                _ => break,
            };
            self.consume();
            let right = self.parse_primary()?;
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }

        Some(left)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek() {
            Some(TOKEN::NUMBERS(_)) => {
                if let Some(TOKEN::NUMBERS(n)) = self.consume() {
                    return Some(Expr::Number(*n));
                }
                None
            }
            Some(TOKEN::LPAREN) => {
                self.consume();
                let expr = self.parse()?;
                if self.consume() != Some(&TOKEN::RPAREN) {
                    return None;
                }
                Some(expr)
            }
            _ => None
        }
    }
}