pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    if message.trim().ends_with("?") &&
        message.chars().any(|x|x.is_ascii_uppercase())
        && !message.chars().any(|x|x.is_ascii_lowercase()) {
            return "Calm down, I know what I'm doing!";
    }
    if message.trim().ends_with("?"){
        return "Sure.";
    }
    if message.chars().any(|x|x.is_ascii_uppercase())
     && !message.chars().any(|x|x.is_ascii_lowercase()){
        return "Whoa, chill out!";
    }
    "Whatever."
}
