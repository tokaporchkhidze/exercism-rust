use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    let mut res = String::new();
    for pair in list.windows(2) {
        writeln!(res, "For want of a {} the {} was lost.", pair[0], pair[1]).unwrap();
    }
    write!(res, "And all for the want of a {}.", list[0]).unwrap();
    res
}
