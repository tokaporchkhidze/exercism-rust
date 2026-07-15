use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::with_capacity(input.len());
    for s in input.graphemes(true).rev() {
        reversed.push_str(s);
    }
    reversed
}
