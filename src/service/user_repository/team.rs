use log::{info, warn};
use sqlx::Row;
use crate::service::user_repository::UserRepository;

impl UserRepository {
    pub(crate) async fn list_teams(&self) -> Vec<String> {
        info!("list_teams");
        let teams = sqlx::query("SELECT name FROM team ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .iter()
            .map(|row| row.get(0))
            .collect();
        teams
    }

    pub(crate) async fn insert_team(&self, team: &str) {
        info!("insert_team {}", team);
        let query = "INSERT INTO team (name) VALUES (?)";
        let result = sqlx::query(query)
            .bind(team)
            .execute(&self.pool)
            .await;
        match result {
            Ok(_) => {},
            Err(err) => warn!("Error inserting team {}: {}", team, err)
        }
    }
}