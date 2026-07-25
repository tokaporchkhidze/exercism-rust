const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

fn get_flower(c: char) -> &'static str {
    match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("Invalid flower letter"),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    if let Some(student_id) = STUDENTS.iter().position(|&s| student == s) {
        let row_id = student_id * 2;
        let mut res = vec![];
        for row in diagram.split('\n') {
            res.push(get_flower(row.as_bytes()[row_id] as char));
            res.push(get_flower(row.as_bytes()[row_id + 1] as char));
        }
        res
    } else {
        panic!("Invalid student name");
    }
}
