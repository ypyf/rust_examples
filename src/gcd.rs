fn main() {
    test(10, 2);
    test(10, 3);
    test(2, 10);
    test(3, 10);
}

fn test(m: u64, n: u64) {
    println!("{}和{}的最大公约数是{}", m, n, gcd(m, n));
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m > 0 && n > 0);
    while n != 0 {
        let t = n;
        n = m % n;
        m = t;
    }
    return m;
}