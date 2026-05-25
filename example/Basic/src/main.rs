use mathrc::calc::frac::Frac;
use mathrc::math::Math;

fn main() {
    println!("2 + 3 = {}", Math::add(2, 3)); // 5
    println!("5 - 2 = {}", Math::sub(5, 2)); // 3
    println!("10 / 2 = {}", Math::div(10, 5)); //5
    println!("2 * 3 = {}", Math::mul(2, 3)); // 6

    println!("e = {}", Math::E); // 2.718...
    println!("PI = {}", Math::PI); // 3.14...

    let frac = Frac::new(2, 4).unwrap();
    println!("1/2={}", frac.normalize()) // 1/2
}
