// Efficient approach - https://iq.opengenus.org/difference-between-square-of-sum-and-sum-of-squares/

// Sum of 1 to N = N * (N+1) / 2
pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2) * (n * (n + 1) / 2)
}

// Sum of square of 1 to N = (2 * N + 1) * (N + 1) / 6
pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
