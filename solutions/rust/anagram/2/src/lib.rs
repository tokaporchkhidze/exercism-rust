use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagram_hash: HashMap<char, i32> = HashMap::new();
    let mut res = HashSet::with_capacity(possible_anagrams.len());
    let lower_input = word.to_lowercase();
    for c in lower_input.chars() {
        *anagram_hash.entry(c).or_default() += 1;
    }

    for &candidate in possible_anagrams {
        let lower_candidate = candidate.to_lowercase();
        if lower_input == lower_candidate {
            continue;
        }
        let mut hash = anagram_hash.clone();
        for c in lower_candidate.chars() {
             *hash.entry(c).or_default() -= 1;
        }
        if hash.values().all(|&count| {count == 0}) {
            res.insert(candidate);
        }
    }
    res
}
