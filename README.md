# MathRC
[![CI](https://github.com/as7ar/mathrc/actions/workflows/rust.yml/badge.svg)](https://github.com/as7ar/mathrc/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/mathrc.svg)](https://crates.io/crates/mathrc)
[![Docs.rs](https://docs.rs/mathrc/badge.svg)](https://docs.rs/mathrc)
[![Downloads](https://img.shields.io/crates/d/mathrc.svg)](https://crates.io/crates/mathrc)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

MathRC is a lightweight Rust mathematics library that provides basic mathematical utilities, fraction handling, functional analysis, and a simple arithmetic expression calculator.

## Installation

Add MathRC to your `Cargo.toml`:
```toml
[dependencies]
mathrc = "0.2.0"
```
Or, if you want to use the Git repository directly:
```toml
[dependencies]
mathrc = { git = "https://github.com/as7ar/mathrc" }
```

## Usage

### Basic arithmetic
```rust
use mathrc::math::Math;

fn main() {
    let sum = Math::add(10, 5);
    let difference = Math::sub(10, 5);
    let product = Math::mul(10, 5);
    let quotient = Math::div(10, 5);

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
}
```

### Mathematical constants
```rust
use mathrc::math::Math;

fn main() {
    println!("PI: {}", Math::PI);
    println!("E: {}", Math::E);
    println!("INF: {}", Math::INF);
}
```

### Square root and absolute value
```rust
use mathrc::math::Math;

fn main() {
    let result = Math::sqrt(16);

    match result {
        Some(value) => println!("sqrt: {}", value),
        None => println!("cannot calculate square root of a negative number"),
    }

    let abs = Math::abs(-10);
    println!("absolute value: {}", abs);
}
```

### Power and logarithm
```rust
use mathrc::math::Math;

fn main() {
    let powered = Math::pow(2, 3);
    let logged = Math::log(8, 2);

    println!("2^3 = {}", powered);

    if let Some(value) = logged {
        println!("log base 2 of 8 = {}", value);
    }
}
```

### GCD and LCM
```rust
use mathrc::math::Math;

fn main() {
    let gcd = Math::gcd(12, 18);
    let lcm = Math::lcm(12, 18);

    println!("gcd: {}", gcd);
    println!("lcm: {}", lcm);
}
```

### Factorial
```rust
use mathrc::math::Math;

fn main() {
    let math = Math;
    let result = math.factorial(5);
    println!("5! = {}", result);
}
```

### Calculator
```rust
use mathrc::calculator::Calculator;

fn main() {
    let result = Calculator::calc("3 + 2 * 4");

    match result {
        Some(value) => println!("result: {}", value),
        None => println!("invalid expression"),
    }
}
```

Supported calculator operators:

| Operator | Description |
| --- | --- |
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `(...)` | Parentheses |

The tokenizer also supports the following LaTeX-style operators:

| Input | Description |
| --- | --- |
| `\times` | Multiplication |
| `\div` | Division |

Example:
```rust
use mathrc::calculator::Calculator;

fn main() {
    let result = Calculator::calc("6 \\div 2");
    assert_eq!(result, Some(3.0));
}
```

### Decimal to fraction
```rust
use mathrc::calculator::Calculator;

fn main() {
    let fraction = Calculator::dec_to_frac(0.5);
    println!("{}", fraction);
}
```

Output:
```text
1/2
```

### Functional Analysis
The `Func` module provides numerical calculus operations including derivatives and integrals.

```rust
use mathrc::function::function::Func;

fn main() {
    let f = Func::new(|x| x * x);
    
    // Evaluate function
    println!("f(5) = {}", f.call(5));
    
    // Calculate derivative
    let df = f.derivative();
    println!("f'(5) = {}", df.call(5));
    
    // Calculate integral
    let integral = Func::new(|x| x).integral();
    println!("∫x dx from 0 to 10 = {}", integral.call(10.0));
}
```

## Status

MathRC is currently in early development. APIs may change as the library grows.

## License

This project is licensed under the GPL-3.0 license.
