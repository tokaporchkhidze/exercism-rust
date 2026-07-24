pub fn brackets_are_balanced(string: &str) -> bool {
    let mut symbols_stack = vec![];
    let check_sym = |c: Option<&char>, expected: char| -> bool {
        c.is_some_and(|&s| { s == expected})
    };
    for c in string.chars() {
        match c {
            ']' => {
                if !check_sym(symbols_stack.last(), '[') {
                    return false;
                }
                symbols_stack.pop();
            }
            ')' => {
                if !check_sym(symbols_stack.last(), '(') {
                    return false;
                }
                symbols_stack.pop();
            }
            '}' => {
                if !check_sym(symbols_stack.last(), '{') {
                    return false;
                }
                symbols_stack.pop();
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
