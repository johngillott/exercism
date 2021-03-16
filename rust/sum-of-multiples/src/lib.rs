pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![false; limit as usize];

    for i in 0..factors.len() {
        for j in 1..limit {
            let p = j * factors[i];
            if p >= limit {
                break;
            }
            multiples[p as usize] = true;
        }
    }

    let mut sum: u32 = 0;

    for i in 1..limit {
        if multiples[i as usize] {
            sum += i;
        }
    }

    sum
}
