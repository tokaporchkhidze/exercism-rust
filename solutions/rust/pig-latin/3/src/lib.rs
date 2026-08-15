pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(convert_to_pig_latin)
        .collect::<Vec<_>>()
        .join(" ")
}

fn convert_to_pig_latin(word: &str) -> String {
    if word.starts_with(['a', 'e', 'i', 'o', 'u'])
        || word.starts_with("xr")
        || word.starts_with("yt")
    {
        return format!("{word}ay");
    }
    let mut split_index = word
        .char_indices()
        .find(|&(i, c)| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') || (i > 0 && c == 'y'))
        .map(|(i, _)| i)
        .unwrap_or(word.len());

    if split_index > 0 && word[split_index..].starts_with('u') && word[..split_index].ends_with('q')
    {
        split_index += 1;
    }
    let (left_part, right_part) = word.split_at(split_index);
    format!("{right_part}{left_part}ay")
}
