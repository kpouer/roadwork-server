use crate::hash;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct User {
    pub(crate) username: String,
    /**
     * The password is salted using BCrypt
     */
    pub(crate) password_hash: String,
    pub(crate) teams: Vec<String>,
    pub(crate) admin: bool,
}

impl User {
    pub(crate) fn is_valid<S: AsRef<str>>(&self, password: S) -> bool {
        debug!("is_valid username={}", self.username);
        let password = password.as_ref();
        hash::check(&self.password_hash, password)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ username: {}, password_hash: XXXX, teams: {:?}, admin: {} }}",
            self.username, self.teams, self.admin
        )
    }
}
