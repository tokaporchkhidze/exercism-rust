pub fn translate(input: &str) -> String {
    if input.starts_with(['a', 'e', 'i', 'o', 'u'])
        || input.starts_with("xr")
        || input.starts_with("yt")
    {
        return format!("{}ay", input);
    }
    let (qu_index, y_index) = (input.find("qu"), input.find('y'));
    match (qu_index, y_index) {
        (Some(qu_index), Some(y_index)) => {
            let split_index = if qu_index < y_index {
                qu_index
            } else {
                y_index.saturating_sub(1)
            };
            let (left_part, right_part) = input.split_at(split_index);
            format!("{}{}ay", right_part, left_part)
        }
        (Some(qu_index), None) => {
            let (_, rest) = input.split_at(qu_index);
            format!("{}quay", rest)
        },
        (None, Some(y_index)) => {
            let (left_part, _) = input.split_at(y_index.saturating_sub(1));
            format!("{}ay", left_part)
        },
        (None, None) => {
            format!("{}ay", input)
        }
    }
}
