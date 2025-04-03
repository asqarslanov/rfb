#[inline(always)]
pub fn validate_email(value: &str) -> bool {
    validator::ValidateEmail::validate_email(&value)
}

#[inline(always)]
pub fn validate_username(value: &str) -> bool {
    let length_check = (3..=25).contains(
        // Since all characters need to be ASCII,
        // the length in bytes should equal the actual length in chars.
        &value.len(),
    );
    length_check && value.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
}

#[inline(always)]
pub fn validate_password(value: &str) -> bool {
    let length_check = (8../* ..=64 */).contains(
        // Since all characters need to be ASCII,
        // the length in bytes should equal the actual length in chars.
        &value.len(),
    );
    length_check
    //     && value
    //         .chars()
    //         .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation())
}
