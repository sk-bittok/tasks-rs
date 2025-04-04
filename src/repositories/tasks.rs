use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Executor, PgPool, Postgres, prelude::FromRow};
use uuid::Uuid;

use crate::models::tasks::NewTask;

use super::ModelError;

#[derive(Debug, Deserialize, Serialize, FromRow, Decode)]
pub struct Task {
    pub id: i32,
    pub pid: Uuid,
    pub user_pid: Uuid,
    pub title: String,
    pub done: bool,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl Task {
    pub async fn create_task(
        db: &PgPool,
        params: &NewTask,
        user_pid: Uuid,
    ) -> Result<Self, ModelError> {
        let item = sqlx::query_as::<_, Self>(
            "
            INSERT INTO tasks (user_pid, title, done)
            VALUES ($1, $2, $3) RETURNING *
            ",
        )
        .bind(user_pid)
        .bind(&params.title)
        .bind(params.done)
        .fetch_one(db)
        .await;

        match item {
            Ok(task) => Ok(task),
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.is_foreign_key_violation() {
                        return Err(ModelError::Unauthorised);
                    }
                }
                Err(e.into())
            }
        }
    }

    pub async fn find_all<'e, C>(db: C, user_pid: Uuid) -> Result<Vec<Self>, ModelError>
    where
        C: Executor<'e, Database = Postgres>,
    {
        let items = sqlx::query_as::<_, Self>("SELECT * FROM tasks WHERE user_pid = $1")
            .bind(user_pid)
            .fetch_all(db)
            .await?;

        Ok(items)
    }
}
