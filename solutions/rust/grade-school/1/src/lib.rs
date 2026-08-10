use std::collections::{BTreeMap, BTreeSet, HashSet};

pub struct School {
    grades_to_students: BTreeMap<u32, BTreeSet<String>>,
    students: HashSet<String>,
}

impl School {
    pub fn new() -> School {
        Self {
            grades_to_students: BTreeMap::new(),
            students: HashSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.students.contains(student) {
            self.grades_to_students
                .entry(grade)
                .or_default()
                .insert(student.to_string());
            self.students.insert(student.to_string());
        }

    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades_to_students.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades_to_students
            .get(&grade)
            .map_or(vec![], |set| set.iter().map(|s| s.to_string()).collect())
    }
}
