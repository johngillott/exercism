pub fn verse(n: u32) -> String {
    match n {
        0 => String::from(
                "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
            ),
        1 => String::from(
            "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
        ),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();

    let num_of_lines: u32 = start - end + 1;

    for i in (0..num_of_lines).rev() {
        song.push_str(&verse(i + end));
        if i != 0 {
            song.push('\n');
        }
    }

    song
}
