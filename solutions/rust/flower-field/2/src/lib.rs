pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(i, &row)| {
            row.bytes()
                .enumerate()
                .map(|(j, point)| populate_point(garden, i, j, point))
                .collect()
        })
        .collect()
}

fn populate_point(garden: &[&str], i: usize, j: usize, point: u8) -> char {
    if point == b'*' {
        return '*';
    }
    let mut counter = 0u8;
    for dir_i in -1..=1isize {
        for dir_j in -1..=1isize {
            if dir_i == 0 && dir_j == 0 {
                continue;
            }
            if is_flower(garden, i as isize + dir_i, j as isize + dir_j) {
                counter += 1;
            }
        }
    }
    match counter {
        0 => ' ',
        n => (b'0' + n) as char,
    }
}

fn is_flower(garden: &[&str], i: isize, j: isize) -> bool {
    if i < 0 || j < 0 {
        return false;
    }
    garden
        .get(i as usize)
        .and_then(|&row| row.as_bytes().get(j as usize))
        == Some(&b'*')
}
