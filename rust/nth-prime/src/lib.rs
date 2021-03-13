pub fn nth(n: u32) -> u32 {
    let mut x = 0;
    let mut primes = 0;

    loop {
        if is_prime(x) {
            primes = primes + 1;
        }

        if primes == n + 1 {
            return x;
        }

        x = x + 1;
    }
}

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for x in 2..n {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}
