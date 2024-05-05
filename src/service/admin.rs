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

    /// Retrieve a user from the repository and check it's password
    /// If the password is missing or wrong the user is not returned
    pub(crate) async fn get_user<S: AsRef<str>>(&self, username: S, password: S) -> Option<User> {
        info!("get_user");
        let user = self.user_repository.find_user(&username).await;
        if let Some(user) = user {
            if user.is_valid(password) {
                info!("get_user -> {}", user.username);
                return Some(user);
            }
            warn!("get_user password is invalid");
            return None;
        }
        warn!("get_user user do not exist {}", username.as_ref());
        None
    }

    pub(crate) async fn change_password<S: AsRef<str>>(&self, user_name: &S, clear_password: &S) -> bool {
        let user_name = user_name.as_ref();
        info!("change_password username={}", user_name);
        let salted_password = hash::salt(clear_password);
        self.user_repository.update_password(user_name, salted_password).await
    }

    /// Check if the user is an admin
    /// It is admin is the user and password are valid and the password has been modified
    pub(crate) async fn is_admin<S: AsRef<str>>(&self, username: &S, password: Option<String>) -> bool {
        info!("is_admin username={}", username.as_ref());
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