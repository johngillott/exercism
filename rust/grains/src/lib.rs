const MIN: u32 = 1;
const MAX: u32 = 64;

pub fn square(s: u32) -> u64 {
    match s {
        MIN..=MAX => 2u64.pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (MIN..=MAX).map(square).sum()
}
