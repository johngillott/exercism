pub fn reverse(input: &str) -> String {
    let l = input.len();

    let mut s = String::with_capacity(l);

    for c in input.chars().rev() {
        s.push(c)
    }

    return s;
}
