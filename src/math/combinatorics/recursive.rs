type int = usize;
fn gcd(a: int, b: int) -> int {
    if b == 0 { return a }
    gcd(b, a % b)
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
pub fn x_gcd(a: int, b: int) -> (int, (int, int)) {
    if b == 0 { return (a, (1, 0)) }
    let (res, (x, y)) = x_gcd(b, a % b);
    (res, (y, x - (a / b) * y))
}
fn x_gcd_wrapper(mut a: usize, mut b: usize) -> int {
    if a < b { (a, b) = (b, a) }
    let (res, _) = x_gcd(a, b);
    res
}

fn factorial(n: int) -> int {
    if n == 0 { return 1 }
    n * factorial(n - 1)
}
fn permutations(n: int, r: int) -> int {
    factorial(n) / factorial(n - r)
}
fn product(n: int, r: int) -> int {
    n.pow(r as u32)
}
fn combinations(n: int, r: int) -> int {
    permutations(n, r) / factorial(r)
}
fn combinations_with_replacement(n: int, r: int) -> int {
    combinations(n + r - 1, r)
}