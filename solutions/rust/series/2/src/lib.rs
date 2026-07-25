pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        panic!("Invalid length")
    }
    let mut vec = vec![];
    for (start, end) in (len - 1..digits.len()).enumerate() {
        let num = &digits[start..=end];
        vec.push(num.to_string());
    }
    vec
}
