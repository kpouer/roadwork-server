use log::info;
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

    pub(crate) async fn insert_team(&self, team: &str) -> Result<(), String> {
        info!("insert_team {}", team);
        let query = "INSERT INTO team (name) VALUES (?)";
        let result = sqlx::query(query).bind(team).execute(&self.pool).await;

        result
            .map(|_| ())
            .map_err(|err| format!("Error inserting team {team}: {err}"))
    }

    pub(crate) async fn delete_team(&self, team: &String) -> Result<(), String> {
        info!("remove_team {team}");
        if self.team_has_users(team).await {
            return Err(format!("Team {team} has users"));
        }
        let query = "DELETE FROM team WHERE name = ?";
        let result = sqlx::query(query).bind(team).execute(&self.pool).await;
        result
            .map(|_| ())
            .map_err(|err| format!("Error removing team {team}: {err}"))
    }

    async fn team_has_users(&self, team: &String) -> bool {
        let query = "SELECT COUNT(*) FROM user_team WHERE team = ?";
        let count = sqlx::query(query)
            .bind(team)
            .fetch_one(&self.pool)
            .await
            .unwrap()
            .get::<i64, _>(0);
        count > 0
    }
}
