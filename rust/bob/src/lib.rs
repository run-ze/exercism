pub fn reply(message: &str) -> &str {
    let message = message.trim();
    match message {
        _ if message.is_empty() => "Fine. Be that way!",
        _ if is_yell(message) => match message {
            _ if message.ends_with('?') => "Calm down, I know what I'm doing!",
            _ => "Whoa, chill out!",
        },
        _ if message.ends_with('?') => "Sure.",
        _ => "Whatever.",
    }
}

fn is_yell(message: &str) -> bool {
    message.to_uppercase() == message && message.chars().any(|x| x.is_alphabetic())
}
