use chrono::{DateTime, FixedOffset, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Executor, PgPool, Postgres, postgres::PgQueryResult, prelude::FromRow};
use uuid::Uuid;

use crate::models::tasks::{NewTask, UpdateTask};

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
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
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

    pub async fn find_by_id<'e, C>(db: C, user_pid: Uuid, id: i32) -> Result<Self, ModelError>
    where
        C: Executor<'e, Database = Postgres>,
    {
        let item = sqlx::query_as::<_, Self>("SELECT * FROM tasks WHERE id = $1 and user_pid = $2")
            .bind(id)
            .bind(user_pid)
            .fetch_optional(db)
            .await?;

        item.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn delete_by_id(
        db: &PgPool,
        id: i32,
        user_pid: Uuid,
    ) -> Result<PgQueryResult, ModelError> {
        let mut txn = db.begin().await?;

        let task = Task::get_by_id(&mut *txn, id)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;

        if task.user_pid != user_pid {
            return Err(ModelError::Unauthorised);
        }

        let query = sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id)
            .execute(&mut *txn)
            .await?;

        if query.rows_affected() > 1 {
            txn.rollback().await?;
            return Err(ModelError::Database("Deleted more than one record".into()));
        }

        txn.commit().await?;

        Ok(query)
    }

    pub async fn update_by_id(
        db: &PgPool,
        params: &UpdateTask,
        id: i32,
        user_pid: Uuid,
    ) -> Result<Self, ModelError> {
        let mut txn = db.begin().await?;

        let tast_to_update = Self::get_by_id(&mut *txn, id)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        if tast_to_update.user_pid != user_pid {
            return Err(ModelError::Unauthorised);
        }

        let title = params.title.as_ref().map_or_else(
            || tast_to_update.title.to_string(),
            |title| title.to_string(),
        );

        let updated_at = Utc::now().fixed_offset();

        let task = sqlx::query_as::<_, Self>(
            "
            UPDATE tasks
            SET title = $3, done = $4, updated_at = $5
            WHERE id = $1 AND user_pid = $2
            RETURNING *",
        )
        .bind(id)
        .bind(user_pid)
        .bind(title)
        .bind(params.done)
        .bind(updated_at)
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(task)
    }

    pub async fn get_by_id<'e, C>(db: C, id: i32) -> Result<Option<Self>, ModelError>
    where
        C: Executor<'e, Database = Postgres>,
    {
        let item = sqlx::query_as::<_, Self>("SELECT * FROM tasks WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(item)
    }
}
