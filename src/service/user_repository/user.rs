use log::{info, warn};
use sqlx::Row;
use crate::model::user::User;
use crate::service::user_repository::UserRepository;

impl UserRepository {
    pub(crate) async fn list_users(&self) -> Vec<String> {
        info!("list_users");
        let users = sqlx::query("SELECT username FROM user ORDER BY username")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .iter()
            .map(|row| row.get(0))
            .collect();
        users
    }

    pub(crate) async fn update_password(&self, user_name: &str, salted_password: String) -> bool {
        info!("update_password {}", user_name);
        let query = "UPDATE user SET password_hash = ? WHERE username = ?";
        let result = sqlx::query(query)
            .bind(&salted_password)
            .bind(user_name)
            .execute(&self.pool)
            .await;
        if result.is_err() {
            warn!("Error updating password for user {}: {}", user_name, result.err().unwrap());
            return false;
        }
        true
    }

    pub(crate) async fn insert_user(&self, user: &User) {
        info!("insert_user {}", user.username);
        let query = "INSERT INTO user (username, password_hash, admin) VALUES (?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&user.username)
            .bind(&user.password_hash)
            .bind(&user.admin)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => {},
            Err(err) => warn!("Error inserting user {} : {}", user, err)
        }
    }

    pub(crate) async fn find_user<S: AsRef<str>>(&self, username: S) -> Option<User> {
        let username = username.as_ref();
        info!("find_user {}", username);
        let query = "SELECT username, password_hash, admin FROM user WHERE username = ?";
        let row = sqlx::query(query)
            .bind(username)
            .fetch_one(&self.pool)
            .await
            .ok();
        match row {
            None => None,
            Some(row) => {
                let teams = self.find_user_teams(username).await;
                Some(User {
                    username: row.get(0),
                    password_hash: row.get(1),
                    teams,
                    admin: row.get(2),
                })
            }
        }
    }
}