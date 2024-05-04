use log::warn;

pub(crate) fn salt<S: AsRef<str>>(password: S) -> String {
    let password = password.as_ref();
    bcrypt::hash(password, 4).unwrap()
}

pub(crate) fn check(expected_password_hash: &str, password: &str) -> bool {
    let result = bcrypt::verify(password, &expected_password_hash);
    return result.unwrap_or_else(|err| {
        warn!("Error checking password: {}", err);
        false
    })
}