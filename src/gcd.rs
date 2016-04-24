fn main() {
    let m = 10;
    let n = 2;
    println!("{}和{}的最大公约数是{}", m, n, gcd(m, n));
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m > 0 && n > 0);
    while m < n {
        let t = m;
        m = n;
        n = t;
        m = m % n;
    }
    n
}
