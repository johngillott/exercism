const MIN: u32 = 1;
const MAX: u32 = 64;

pub fn square(s: u32) -> u64 {
    match s {
        MIN..=MAX => (1..u64::from(s)).map(|x| 2 * x).sum(),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (MIN..MAX + 1).map(square).sum()
}
