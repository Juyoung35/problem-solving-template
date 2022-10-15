type int = usize;
fn gcd(mut a: int, mut b: int) -> int {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn gcd_wrapper(mut a: int, mut b: int) -> int {
    if a < b { (a, b) = (b, a) }
    gcd(a, b)
}
fn lcm(a: int, b: int) -> int {
    (a * b) / gcd(a, b)
}
fn lcm_wrapper(mut a: int, mut b: int) -> int {
    if a < b { (a, b) = (b, a) }
    lcm(a, b)
}
use crate::math::combinatorics::recursive::x_gcd;
fn x_gcd_wrapper(mut a: usize, mut b: usize) -> int {
    if a < b { (a, b) = (b, a) }
    let (res, _) = x_gcd(a, b);
    res
}

fn factorial(n: int) -> int {
    (1..=n).fold(1, |prod, i| prod * i)
}