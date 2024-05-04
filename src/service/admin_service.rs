use log::{info, warn};
use crate::hash;
use crate::model::user::User;
use crate::service::user_repository::UserRepository;

#[derive(Clone)]
pub(crate) struct AdminService {
    user_repository: UserRepository
}

impl AdminService {
    pub(crate) async fn new(user_repository: UserRepository) -> Self {
        AdminService { user_repository }
    }

    pub(crate) async fn change_password<S: AsRef<str>>(&self, user_name: S, clear_password: S) -> bool {
        let user_name = user_name.as_ref();
        info!("change_password username={}", user_name);
        let salted_password = hash::salt(clear_password);
        self.user_repository.update_password(user_name, salted_password).await
    }

    pub(crate) async fn is_valid(&self, username: &str, password: Option<String>) -> bool {
        info!("is_valid username={}", username);
        if let Some(password) = password {
            if let Some(user) = self.user_repository.find_user(username).await {
                if user.is_valid(password.as_str()) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if the user is an admin
    /// It is admin is the user and password are valid and the password has been modified
    pub(crate) async fn is_admin(&self, username: &str, password: Option<String>) -> bool {
        info!("is_admin username={}", username);
        if let Some(password) = password {
            if password == "admin" {
                warn!("Password was not modified!");
                return false;
            }
            if let Some(user) = self.user_repository.find_user(username).await {
                if user.is_valid(password.as_str()) {
                    return user.admin;
                }
            }
        }
        false
    }

    async fn find_valid_user(&self, username: &str, password: &str) -> Option<User> {
        if let Some(user) = self.user_repository.find_user(username).await {
            if user.is_valid(password) {
                return Some(user);
            }
        }
        None
    }
}