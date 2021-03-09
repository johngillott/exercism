use unicode_segmentation::UnicodeSegmentation;

/*
TIL Segmentation into Unicode grapheme clusters was supported by
the standard library however was removed due to the size of the required
Unicode tables. [source: https://stackoverflow.com/a/58770681]

crate: https://docs.rs/unicode-segmentation/1.7.1/unicode_segmentation/struct.Graphemes.html
*/

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<std::string::String>()
}
