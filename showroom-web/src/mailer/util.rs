use aws_sdk_sesv2::{
    error::{DisplayErrorContext, ProvideErrorMetadata, SdkError},
    types::Content,
};

pub fn convert_ses_content(data: impl Into<String>) -> Content {
    Content::builder().data(data.into()).build().unwrap()
}

pub fn ses_error<E, R>(action: &str, error: &SdkError<E, R>) -> String
where
    E: ProvideErrorMetadata + std::error::Error + 'static,
    R: std::fmt::Debug,
{
    match error.message() {
        Some(message) => format!("{action}: {message}"),
        None => format!("{action}: {}", DisplayErrorContext(error)),
    }
}

pub fn is_retryable<E: ProvideErrorMetadata, R>(error: &SdkError<E, R>) -> bool {
    match error {
        SdkError::TimeoutError(_) | SdkError::DispatchFailure(_) | SdkError::ResponseError(_) => true,
        _ => matches!(
            error.code(),
            Some("TooManyRequestsException" | "Throttling" | "ThrottlingException" | "InternalFailure" | "ServiceUnavailable")
        ),
    }
}

const MAX_ENCODED_WORD: usize = 75;

pub fn from_address(name: &str, email: &str) -> String {
    let name = name.split(|character: char| character.is_whitespace() || character.is_control()).filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" ");
    if name.is_empty() {
        return email.to_string();
    }
    if name.is_ascii() {
        return format!("\"{}\" <{email}>", name.replace('\\', "\\\\").replace('"', "\\\""));
    }
    format!("{} <{email}>", encoded_words(&name))
}

fn encoded_words(text: &str) -> String {
    let prefix = "=?UTF-8?Q?";
    let suffix = "?=";
    let mut words = Vec::new();
    let mut word = String::new();
    let mut buffer = [0u8; 4];

    for character in text.chars() {
        let encoded: String = character
            .encode_utf8(&mut buffer)
            .bytes()
            .map(|byte| match byte {
                b' ' => "_".to_string(),
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'!' | b'*' | b'+' | b'-' | b'/' => char::from(byte).to_string(),
                _ => format!("={byte:02X}"),
            })
            .collect();
        if prefix.len() + word.len() + encoded.len() + suffix.len() > MAX_ENCODED_WORD {
            words.push(format!("{prefix}{word}{suffix}"));
            word.clear();
        }
        word.push_str(&encoded);
    }
    words.push(format!("{prefix}{word}{suffix}"));
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_ascii_names_and_encodes_the_rest() {
        assert_eq!(from_address("Ava's \"room\", etc.", "a@room.lc"), "\"Ava's \\\"room\\\", etc.\" <a@room.lc>");
        assert_eq!(from_address("Café\nnews", "a@room.lc"), "=?UTF-8?Q?Caf=C3=A9_news?= <a@room.lc>");
    }
}
