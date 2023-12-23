pub fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
