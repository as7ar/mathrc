use mathrc::sequence::seq::Seq;
use mathrc::sequence::sequences::factorial;
use mathrc::sequence::sum::sum;

fn main() {
    let a = Seq::define(|n| n as f64 + 1f64);
    println!("a_1 = {}", a.nth(1)); // 2
    println!("S_10 = {}", a.sum(1, 10)); // 65

    println!("10! = {}", factorial(5)); // 120

    let sum = sum(2, 11, |x| (x as f64).powi(2));
    println!("sum(x^2 2 to 11) = {}", sum) // 505
}
