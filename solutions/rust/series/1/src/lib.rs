pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        panic!("Invalid length")
    }
    let mut vec = vec![];
    let mut start = 0;
    for end in len - 1..digits.len() {
        let num = &digits[start..=end];
        vec.push(num.to_string());
        start += 1;
    }
    vec
}
