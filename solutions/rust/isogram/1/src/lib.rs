use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate.chars().filter(|c| *c != ' ' && *c != '-').all(move |c|
        set.insert(c.to_ascii_lowercase())
    )
}
