use crate::Error::{InvalidDigit, InvalidInputBase, InvalidOutputBase};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(InvalidInputBase);
    }
    if to_base < 2 {
        return Err(InvalidOutputBase);
    }
    if number.is_empty() {
        return Ok(vec![0]);
    }
    let mut res = vec![];
    let mut num = 0;
    for (i, &d) in number.iter().enumerate() {
        if d >= from_base {
            return Err(InvalidDigit(d));
        }
        num += d * from_base.pow((number.len() - i - 1) as u32);
    }

    while num > 0 {
        res.insert(0, num % to_base);
        num /= to_base;
    }

    let mut start = 0;
    while start < res.len() {
        if res[start] != 0 {
            break;
        }
        start += 1;
    }

    let res = res[start..].to_vec();

    Ok(if res.is_empty() { vec![0] } else { res })
}
