pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let num_digits = str.len() as u32;
    str.chars().fold(0, |acc, num| {
        acc + num.to_digit(10).unwrap().pow(num_digits)
    }) == num
}
