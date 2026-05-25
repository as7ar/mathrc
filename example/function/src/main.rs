use mathrc::function::function::Func;

fn main() {
    let f = Func::new(|x| x.powi(2));

    println!("f(2)={}", f.call(2f64));

    println!("f'(2)={}", f.clone().derivative().call(2f64));
    println!("f'(2)={}", f.clone().derivative().call(2f64))
}
