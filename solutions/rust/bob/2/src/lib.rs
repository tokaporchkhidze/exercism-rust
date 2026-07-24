fn is_alphabetic_shouting(message: &str) -> bool {
    let mut iter = message
        .chars()
        .filter(|c| c.is_alphabetic()).peekable();
    iter.peek().is_some() && iter.all(char::is_uppercase)
}
pub fn reply(message: &str) -> &str {
    let message = message.trim();

    match (is_alphabetic_shouting(message), message.ends_with("?"), message.is_empty()) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (false, true, _) => "Sure.",
        (true, false, _) => "Whoa, chill out!",
        (false, false, _) => "Whatever.",
    }
}
