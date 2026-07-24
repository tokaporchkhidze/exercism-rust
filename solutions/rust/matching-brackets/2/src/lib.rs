pub fn brackets_are_balanced(string: &str) -> bool {
    let mut symbols_stack = vec![];
    for c in string.chars() {
        match c {
            ']' => {
                if symbols_stack.pop() != Some('[') {
                    return false;
                }
            }
            ')' => {
                if symbols_stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if symbols_stack.pop() != Some('{') {
                    return false;
                }
            }
            '[' | '{' | '(' => {
                symbols_stack.push(c);
                continue;
            }
            _ => continue,
        }
    }
    symbols_stack.is_empty()
}
