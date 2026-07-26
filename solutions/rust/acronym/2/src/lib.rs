
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .filter(|&word| !word.is_empty())
        .filter_map(|word| {
            word.find(|c: char| char::is_ascii_alphabetic(&c))
                .map(|id| &word[id..])
        })
        .fold(String::new(), |mut acc, word| {
            let mut prev = word.chars().next().unwrap().to_ascii_uppercase();
            acc.push(prev);
            for c in word.chars() {
                if c.is_ascii_uppercase() && !char::is_ascii_uppercase(&prev) {
                    acc.push(c.to_ascii_uppercase());
                }
                prev = c;
            }
            acc
        })
}
