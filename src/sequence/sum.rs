/// Computes the summation of a function over an integer range.
///
/// Evaluates:
///
/// ```text
/// f(k) + f(k+1) + ... + f(n)
/// = Σ(i=k→n) f(i)
/// ```
///
/// # Arguments
///
/// * `k` - Starting index (inclusive).
/// * `n` - Ending index (inclusive).
/// * `f` - Function to evaluate for each index.
///
/// # Returns
///
/// The sum of all evaluated terms as `f64`.
///
/// # Examples
///
/// ```rust
/// use mathrc::sum;
///
/// let result = sum(1, 5, |n| n as f64);
///
/// assert_eq!(result, 15.0);
/// ```
///
/// ```rust
/// use mathrc::sum;
///
/// let result = sum(1, 3, |n| (n * n) as f64);
///
/// assert_eq!(result, 14.0);
/// ```
pub fn sum<F>(k: u64, n: u64, f: F) -> f64
where
    F: Fn(u64) -> f64,
{
    (k..=n).map(f).sum()
}
