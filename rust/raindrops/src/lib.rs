pub fn raindrops(n: u32) -> String {
    // has 3 as a factor, add 'Pling' to the result.
    // has 5 as a factor, add 'Plang' to the result.
    // has 7 as a factor, add 'Plong' to the result.
    // does not have any of 3, 5, or 7 as a factor, the result should be the digits of the number.
    let mut s = String::new();

    if n % 3 == 0 {
        s.push_str("Pling");
    }

    if n % 5 == 0 {
        s.push_str("Plang");
    }

    if n % 7 == 0 {
        s.push_str("Plong");
    }

    if s.is_empty() {
        s = n.to_string();
    }
    s
}
