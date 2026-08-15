pub fn translate(input: &str) -> String {
    let mut result = String::new();
    for word in input.split_whitespace() {
        let pigged = if word.starts_with(['a', 'e', 'i', 'o', 'u'])
            || word.starts_with("xr")
            || word.starts_with("yt")
        {
            format!("{}ay", word)
        } else {
            let (qu_index, y_index, vowel_index) = (
                word.find("qu"),
                word.find('y'),
                word.find(['a', 'e', 'i', 'o', 'u']),
            );
            match (qu_index, y_index, vowel_index) {
                (Some(qu_index), Some(y_index), _)
                if qu_index > 0
                    && y_index > 0
                    && (qu_index < vowel_index.unwrap_or(usize::MAX)
                    || y_index < vowel_index.unwrap_or(usize::MAX)) =>
                    {
                        let split_index = if qu_index < y_index {
                            qu_index + 2
                        } else {
                            y_index
                        };
                        let (left_part, right_part) = word.split_at(split_index);
                        format!("{}{}ay", right_part, left_part)
                    }
                (Some(qu_index), None, _) if qu_index < vowel_index.unwrap_or(usize::MAX) => {
                    let (left_part, right_part) = word.split_at(qu_index + 2);
                    format!("{}{}ay", right_part, left_part)
                }
                (None, Some(y_index), _) if y_index > 0 && y_index < vowel_index.unwrap_or(usize::MAX) => {
                    let (left_part, right_part) = word.split_at(y_index);
                    format!("{}{}ay", right_part, left_part)
                }
                (_, _, Some(vowel_index)) => {
                    let (left_part, right_part) = word.split_at(vowel_index);
                    format!("{}{}ay", right_part, left_part)
                }
                (_, _, _) => panic!("Invalid word"),
            }
        };
        if result.is_empty() {
            result = pigged;
        } else {
            result = format!("{} {}", result, pigged);
        }
    }
    result

}
