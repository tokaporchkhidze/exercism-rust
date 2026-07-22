/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let stripped_code = code.replace(" ", "");
    if stripped_code.len() <= 1 {
        return false
    }
    let mut sum = 0;
    for (i, val) in stripped_code.chars().rev().enumerate() {
        let mut num = match val {
            c if val.is_ascii_digit() => {(c as u8 - b'0') as i32}
            _ => {return false}
        };
        if i % 2 == 0 {
            sum += num;
        } else {
            num *= 2;
            if num > 9 {
                num -= 9;
            }
            sum += num;
        }
    }

    sum % 10 == 0

}
