pub fn factors(n: u64) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();

    let mut m = n;

    for i in 2..=n {
        while m % i == 0 {
            m /= i;

            v.push(i);
        }
    }

    v
}
