#[derive(Debug, PartialEq, Clone)]
pub enum TOKEN {
    NUMBERS(f64),
    PLUS, // +
    MINUS, // -
    DIVIDE, // /, \\div
    MULTIPLE, // *, \\times
    LPAREN, // (
    RPAREN // )


}

pub fn tokenize(text: &str) -> Option<Vec<TOKEN>> {
    let mut tokens = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(&c) = chars.peek(){
        match c {
            ' ' => {chars.next();}
            '0'..'9' | '.' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n = num.parse::<f64>().map_err(|_| format!("invalid number: {}", num)).ok()?;
                tokens.push(TOKEN::NUMBERS(n));
            }
            '+' => { tokens.push(TOKEN::PLUS); chars.next(); }
            '-' => { tokens.push(TOKEN::MINUS); chars.next(); }
            '*' => { tokens.push(TOKEN::MULTIPLE); chars.next(); }
            '/' => { tokens.push(TOKEN::DIVIDE); chars.next(); }
            '(' => { tokens.push(TOKEN::LPAREN); chars.next(); }
            ')' => { tokens.push(TOKEN::RPAREN); chars.next(); }
            '\\' => {
                chars.next();
                let mut cmd = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() { cmd.push(c); chars.next(); }
                    else { break; }
                }
                match cmd.as_str() {
                    "times" => tokens.push(TOKEN::MULTIPLE),
                    "div" => tokens.push(TOKEN::DIVIDE),
                    _ => return None,
                }
            }
            _ => return None
        }
    }

    Some(tokens)
}