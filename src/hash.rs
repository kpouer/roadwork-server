use log::warn;

pub(crate) fn salt(password: &String) -> String {
    bcrypt::hash(password, 4).unwrap()
}

pub(crate) fn check(expected_password_hash: &str, password: &str) -> bool {
    let result = bcrypt::verify(password, &expected_password_hash);
    return result.unwrap_or_else(|err| {
        warn!("Error checking password: {}", err);
        false
    })
}