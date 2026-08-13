/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut chars = [0; 26];
    sentence
        .chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| chars[c.to_ascii_lowercase() as usize - 'a' as usize] += 1);
    chars.iter().all(|&c| c >= 1)
}
