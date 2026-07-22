/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|&c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 0 { num } else { num * 2 })
                .map(|num| {if num > 9 {num - 9} else {num}})
                .map(|num| {(sum + num, count + 1)})
        }).is_some_and(|(sum, count)| { sum % 10 == 0 && count > 1})
}
