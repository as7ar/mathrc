pub fn sum<F>(
    k: u64, n: u64,
    f: F
) -> f64 where F: Fn(u64)-> f64, {
    (k..=n).map(f).sum()
}