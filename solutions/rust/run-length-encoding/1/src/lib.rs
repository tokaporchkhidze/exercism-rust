pub fn encode(source: &str) -> String {
    let mut chars = source.chars().peekable();
    let mut result = String::new();
    while let Some(c) = chars.next() {
        let mut count = 1;

        while chars.peek() == Some(&c) {
            chars.next();
            count += 1;
        }

        if count > 1 {
            result.push_str(&count.to_string());
        }
        result.push(c);
    }
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    for c in source.chars() {
        if let Some(n) = c.to_digit(10) {
            count = count * 10 + n;
        } else {
            if count > 0 {
                result.extend(std::iter::repeat_n(c, count as usize));
                count = 0;
            } else {
                result.push(c);
            }
        }
    }
    result
}
