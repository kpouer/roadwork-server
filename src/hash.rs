use log::{debug, warn};

pub(crate) fn salt(password: &str) -> String {
    bcrypt::hash(password, 4).unwrap()
}

pub(crate) fn check(expected_password_hash: &str, password: &str) -> bool {
    let result = bcrypt::verify(password, expected_password_hash);
    result
        .inspect(|result| debug!("Password is {}", if *result { "correct" } else { "incorrect" }))
        .inspect_err(|e| warn!("Error checking password: {e}"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_salt() {
        let password = "password";
        let salted_password = salt(password);
        assert!(check(&salted_password, password));
    }

    #[test]
    fn test_salt_wrong_password() {
        let password = "password";
        let salted_password = salt(password);
        assert!(!check(&salted_password, "wrong password"));
    }
}