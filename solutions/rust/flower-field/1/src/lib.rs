pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut res = Vec::with_capacity(garden.len());
    for (i, &row) in garden.iter().enumerate() {
        let mut populated_row = String::with_capacity(row.len());
        for (j, &c) in row.as_bytes().iter().enumerate() {
            if c == b'*' {
                populated_row.push(c as char);
            } else {
                let mut counter = 0;
                // right check
                if j + 1  < row.len() && row.as_bytes()[j + 1] == b'*' {
                    counter += 1;
                }
                // left check
                if j != 0 && row.as_bytes()[j - 1] == b'*' {
                    counter += 1;
                }
                // below check
                if i + 1 < garden.len() && garden[i + 1].as_bytes()[j] == b'*' {
                    counter += 1;
                }
                // above check
                if i != 0 && garden[i - 1].as_bytes()[j] == b'*' {
                    counter += 1;
                }
                // diagonal left above
                if i != 0 && j != 0 && garden[i - 1].as_bytes()[j - 1] == b'*' {
                    counter += 1;
                }
                // diagonal right above
                if i != 0  && j + 1 < garden[i - 1].len() && garden[i - 1].as_bytes()[j + 1] == b'*' {
                    counter += 1;
                }
                // diagonal left below
                if i + 1 < garden.len() && j != 0 && garden[i + 1].as_bytes()[j - 1] == b'*' {
                    counter += 1;
                }
                // diagonal right below
                if i + 1 < garden.len() && j + 1 < garden[i + 1].len() && garden[i + 1].as_bytes()[j + 1] == b'*' {
                    counter += 1;
                }

                if counter > 0 {
                    populated_row.push_str(&counter.to_string());
                } else {
                    populated_row.push(' ');
                }

            }
        }
        res.push(populated_row);
    }
    res
}
