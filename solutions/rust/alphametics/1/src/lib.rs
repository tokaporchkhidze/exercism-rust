use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut sides = input.split("==");
    let (Some(left), Some(right), None) = (sides.next(), sides.next(), sides.next()) else {
        return None;
    };
    let mut coeffs: HashMap<char, (i64, bool)> = HashMap::new();
    let mut add_word = |word: &str, sign: i64| {
        let word = word.trim();
        let multi = word.chars().count() > 1;
        let leading = word.chars().next();
        for (i, c) in word.chars().rev().enumerate() {
            let e = coeffs.entry(c).or_insert((0, false));
            e.0 += sign * 10i64.pow(i as u32);
            e.1 |= multi && leading == Some(c);
        }
    };

    for addend in left.split('+') {
        add_word(addend, 1);
    }
    add_word(right, -1);
    if coeffs.len() > 10 {
        return None;
    }
    let mut coeffs_vec = coeffs
        .iter()
        .map(|(&c, &(coefficient, is_leading))| (c, coefficient, is_leading))
        .collect::<Vec<_>>();
    coeffs_vec.sort_unstable_by_key(|&(_, coeff, _)| {coeff});
    let mut path = Vec::new();
    let mut counter = 0u64;
    if backtrack(&coeffs_vec, &mut path, &mut [false; 10], 0, &mut counter) {
        println!("{} steps",  counter );
        Some(path.into_iter().collect())
    } else {
        None
    }
}

fn backtrack(
    input: &[(char, i64, bool)],
    path: &mut Vec<(char, u8)>,
    used: &mut [bool; 10],
    sum: i64,
    counter: &mut u64,
) -> bool {
    let Some(&(c, coefficient, is_leading)) = input.first() else {
        return sum == 0;
    };
    for j in 0..10 {
        *counter += 1;
        if used[j as usize] || (j == 0 && is_leading) {
            continue;
        }
        used[j as usize] = true;
        path.push((c, j));
        if backtrack(&input[1..], path, used, sum + coefficient * j as i64, counter) {
            return true;
        }
        path.pop();
        used[j as usize] = false;
    }
    false
}
