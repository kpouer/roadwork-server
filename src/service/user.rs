use crate::hash;
use crate::service::user_repository::UserRepository;
use log::{debug, info, warn};
use roadwork_sync_lib::user::User;

#[derive(Clone)]
pub(crate) struct AdminService {
    user_repository: UserRepository,
}

impl AdminService {
    pub(crate) async fn new(user_repository: UserRepository) -> Self {
        AdminService { user_repository }
    }

    /// Retrieve a user from the repository and check it's password
    /// If the password is missing or wrong the user is not returned
    pub(crate) async fn get_user<S: AsRef<str>>(&self, username: &S, password: &S) -> Option<User> {
        debug!("get_user -> {}", username.as_ref());
        let user = self.user_repository.find_user(&username).await;
        if let Some(user) = user {
            debug!("get_user : found {:?}", user);
            if self.is_valid(&user, password) {
                debug!("get_user password is valid");
                return Some(user);
            }
            warn!("get_user password is invalid");
            return None;
        }
        warn!("get_user user do not exist {}", username.as_ref());
        None
    }

    pub(crate) async fn change_password<S: AsRef<str>>(
        &self,
        user_name: &S,
        clear_password: &String,
    ) -> Result<(), String> {
        let user_name = user_name.as_ref();
        info!("change_password username={}", user_name);
        let salted_password = hash::salt(clear_password);
        self.user_repository
            .update_password(user_name, salted_password)
            .await
    }

    /// Check if the user is an admin
    /// It is admin is the user and password are valid and the password has been modified
    pub(crate) async fn is_admin(&self, username: &String, password: &String) -> bool {
        info!("is_admin username={}", username);
        if password == "admin" {
            warn!("Password was not modified!");
            return false;
        }
        if let Some(user) = self.user_repository.find_user(username).await {
            if self.is_valid(&user, password.as_str()) {
                return user.admin;
            }
        }
        false
    }

    pub(crate) async fn has_team(
        &self,
        username: &String,
        password: &String,
        team: &String,
    ) -> bool {
        if let Some(user) = self.find_valid_user(username, password).await {
            if user.teams.contains(team) {
                return true;
            }
        }
        false
    }

    async fn find_valid_user<S: AsRef<str>>(
        &self,
        username: &S,
        password: &String,
    ) -> Option<User> {
        debug!("find_valid_user -> {}", username.as_ref());
        if let Some(user) = self.user_repository.find_user(username).await {
            debug!("find_valid_user : found {:?}", user);
            if self.is_valid(&user, password) {
                debug!("find_valid_user password is valid");
                return Some(user);
            }
            debug!("find_valid_user password is invalid");
        }
        None
    }

    fn is_valid<S: AsRef<str>>(&self, user: &User, password: S) -> bool {
        debug!("is_valid username={}", user.username);
        let password = password.as_ref();
        hash::check(&user.password_hash, password)
    }
}
