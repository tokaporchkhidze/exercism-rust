fn is_alphabetic_shouting(message: &str) -> bool {
    let mut iter = message
        .chars()
        .filter(|c| c.is_alphabetic()).peekable();
    iter.peek().is_some() && iter.all(char::is_uppercase)
}
pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        "Fine. Be that way!"
    } else if message.ends_with("?")
        && is_alphabetic_shouting(message)
    {
        "Calm down, I know what I'm doing!"
    } else if is_alphabetic_shouting(message) {
        "Whoa, chill out!"
    } else if message.ends_with("?") {
        "Sure."
    } else {
        "Whatever."
    }
}
