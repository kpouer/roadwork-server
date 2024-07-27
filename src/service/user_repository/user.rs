use log::info;
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

    pub(crate) async fn update_password(
        &self,
        user_name: &str,
        salted_password: String,
    ) -> Result<(), String> {
        info!("update_password {user_name}");
        let query = "UPDATE user SET password_hash = ? WHERE username = ?";
        let result = sqlx::query(query)
            .bind(&salted_password)
            .bind(user_name)
            .execute(&self.pool)
            .await;
        result
            .map(|_| ())
            .map_err(|err| format!("Error updating password for user {}: {}", user_name, err))
    }

    pub(crate) async fn find_user<S: AsRef<str>>(&self, username: S) -> Option<User> {
        let username = username.as_ref();
        info!("find_user {username}");
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

    pub(crate) async fn insert_user(&self, user: &User) -> Result<(), String> {
        info!("insert_user {}", user.username);
        let query = "INSERT INTO user (username, password_hash, admin) VALUES (?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&user.username)
            .bind(&user.password_hash)
            .bind(user.admin)
            .execute(&self.pool)
            .await;
        result
            .map(|_| ())
            .map_err(|err| format!("Error inserting user {user:?}: {err}"))
    }

    pub(crate) async fn delete_user(&self, username: &String) -> Result<(), String> {
        info!("delete_user {username}");
        self.remove_all_user_teams(username).await?;
        let query = "DELETE FROM user WHERE username = ?";
        let result = sqlx::query(query)
            .bind(username)
            .fetch_all(&self.pool)
            .await;
        result
            .map(|_| ())
            .map_err(|err| format!("Error delete_user {username} : {err}"))
    }

    async fn remove_all_user_teams(&self, username: &String) -> Result<(), String> {
        info!("remove_all_user_teams {username}");
        let query = "DELETE FROM user_team WHERE username = ?";
        let result = sqlx::query(query).bind(username).execute(&self.pool).await;
        result
            .map(|_| ())
            .map_err(|err| format!("Error removin all teams from user {username} : {err}"))
    }
}
