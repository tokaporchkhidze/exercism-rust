fn split_word_by_case(word: &str) -> String {
    let word: String = word.chars().filter(char::is_ascii_alphabetic).collect();
    let mut res = String::from(word.chars().next().unwrap().to_ascii_uppercase());
    if word.chars().all(|c| char::is_ascii_uppercase(&c)) {
        return res;
    }
    for c in word.chars().skip(1) {
        if c.is_ascii_uppercase() {
            res.push(c.to_ascii_uppercase());
        }
    }
    res
}
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .fold(String::new(), |acc, g| {
            if !g.is_empty() {
                acc + &split_word_by_case(g)
            } else {
                acc
            }
        })
}
