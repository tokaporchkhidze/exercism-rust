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
        diagram
            .lines()
            .flat_map(|line| line[row_id..=row_id + 1].chars().map(get_flower))
            .collect()
    } else {
        panic!("Invalid student name");
    }
}
