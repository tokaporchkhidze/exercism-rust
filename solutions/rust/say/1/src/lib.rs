
fn num_to_word(num: u64) -> String {
    match num {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
         8 => "eight".to_string(),
         9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        100 => "hundred".to_string(),
        1000 => "thousand".to_string(),
        1_000_000 => "million".to_string(),
        1_000_000_000 => "billion".to_string(),
        1_000_000_000_000 => "trillion".to_string(),
        1_000_000_000_000_000 => "quadrillion".to_string(),
        1_000_000_000_000_000_000 => "quintillion".to_string(),
        _ => unreachable!(),
    }
}

pub fn encode(n: u64) -> String {
    match n {
        0..=19 => num_to_word(n),
        20..=99 => {
            let tens_count = (n / 10) * 10;
            let leftover = n % 10;
            if leftover == 0 {
                num_to_word(tens_count).to_string()
            } else {
                format!("{}-{}", num_to_word(tens_count), num_to_word(leftover))
            }
        }
        100..=999 => {
            let hundreds_count = n / 100;
            let leftover = n % 100;
            if leftover == 0 {
                return format!("{} hundred", num_to_word(hundreds_count));
            }
            format!("{} hundred {}", num_to_word(hundreds_count), encode(leftover))
        }
        _ => {
            let big_numbers = [
                (1_000_000_000_000_000_000, "quintillion"),
                (1_000_000_000_000_000, "quadrillion"),
                (1_000_000_000_000, "trillion"),
                (1_000_000_000, "billion"),
                (1_000_000, "million"),
                (1_000, "thousand")
            ];
            let &(value, num_word) = big_numbers.iter().find(|&(val, _)| {n >= *val} ).expect("Failures are covered by above cases");
            let count = n / value;
            let leftover = n % value;
            if leftover == 0 {
                format!("{} {}", encode(count), num_word)
            } else {
                format!("{} {} {}", encode(count), num_word, encode(leftover))
            }
        }
    }
}
