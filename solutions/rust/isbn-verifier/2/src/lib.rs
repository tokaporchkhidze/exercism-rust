/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut multiplier: u32 = 10;
    isbn.chars()
        .try_fold(0, |acc, c| match c {
            '0'..='9' if multiplier >= 1 => {
                let num = c.to_digit(10)?;
                let additive = num * multiplier;
                multiplier -= 1;
                Some(acc + additive)
            }
            'X' if multiplier == 1 => {
                multiplier -= 1;
                Some(acc + 10)
            }
            '-' => Some(acc),
            _ => None,
        })
        .is_some_and(|acc| multiplier == 0 && acc % 11 == 0)
}
