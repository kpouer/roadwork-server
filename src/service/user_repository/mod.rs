mod user;
mod team;

use std::fs;
use std::path::Path;

use log::{info, warn};
use sqlx::{Error, Executor, Pool, Row, Sqlite, SqlitePool};

use crate::hash;
use crate::model::user::User;

#[derive(Clone)]
pub(crate) struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub(crate) async fn new() -> Result<Self, sqlx::Error> {
        let (pool, should_init_db) = get_database_pool().await?;
        let repository = UserRepository {
            pool
        };

        if should_init_db {
            repository.init_db().await;
        }
        Ok(repository)
    }

    async fn init_db(&self) {
        let query = "
        CREATE TABLE team (
            name TEXT PRIMARY KEY
        );
        CREATE TABLE user (
            username TEXT PRIMARY KEY,
            password_hash TEXT NOT NULL,
            admin BOOLEAN NOT NULL DEFAULT FALSE
        );
        CREATE TABLE user_team (
            username TEXT NOT NULL,
            team TEXT NOT NULL,
            PRIMARY KEY (username, team),
            FOREIGN KEY (username) REFERENCES user(username),
            FOREIGN KEY (team) REFERENCES team(name)
        );
    ";
        self.pool.execute(query).await.unwrap();
        info!("Database initialized");
        self.init_admin_user().await;
    }

    async fn init_admin_user(&self) {
        info!("init_admin");
        let team = "admin";
        let username = "admin";
        let password = "admin";
        self.insert_team(team).await;
        let admin_user = User {
            username: username.to_string(),
            password_hash: hash::salt(&password.to_string()),
            teams: vec![team.to_string()],
            admin: true,
        };

        self.insert_user(&admin_user).await;
        self.link_user_team(username, team).await;
    }

    pub(crate) async fn is_valid_for_team(&self, username: &str, password: Option<String>, team: &String) -> bool {
        self.is_user_valid(username, password).await && self.has_team(username, team).await
    }

    pub(crate) async fn is_user_valid(&self, username: &str, password: Option<String>) -> bool {
        if password.is_none() {
            warn!("Password is None");
            return false;
        }
        let password = &password.unwrap();
        return match self.find_user(username).await {
            None => {
                warn!("User not found {}", username);
                false
            }
            Some(user) => {
                info!("User found {}", username);
                user.is_valid(password)
            }
        };
    }

    pub(crate) async fn has_team(&self, username: &str, team: &String) -> bool {
        match self.find_user(username).await {
            None => {
                warn!("User not found {}", username);
                false
            }
            Some(user) => {
                info!("User found {}", username);
                user.teams.contains(&team.to_string())
            }
        }
    }

    async fn find_user_teams(&self, username: &str) -> Vec<String> {
        let query = "SELECT DISTINCT team FROM user_team WHERE username = ?";
        let rows = sqlx::query(query)
            .bind(username)
            .fetch_all(&self.pool)
            .await;
        match rows {
            Ok(rows) => return rows.iter().map(|row| row.get(0)).collect(),
            Err(err) => warn!("Error fetching teams for user {}: {}", username, err)
        }

        vec![]
    }

    async fn link_user_team(&self, username: &str, team: &str) {
        info!("link_user_team {} {}", username, team);
        let query = "INSERT INTO user_team (username, team) VALUES (?, ?)";
        let result = sqlx::query(query)
            .bind(username)
            .bind(team)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => {},
            Err(err) => warn!("Error linking user {} with team {}: {}", username, team, err)
        }
    }
}

async fn get_database_pool() -> Result<(Pool<Sqlite>, bool), Error> {
    let db_path = "database/users";
    let should_init_db = !Path::new(db_path).exists();
    if should_init_db {
        fs::create_dir_all("database").unwrap();
        fs::File::create(db_path).expect("Unable to create database file");
    }
    let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await?;
    Ok((pool, should_init_db))
}


