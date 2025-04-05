use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::repositories::tasks::Task;

#[derive(Debug, Deserialize, Clone, ToSchema, Validate)]
pub struct NewTask {
    #[validate(length(
        min = 5,
        max = 255,
        message = "Title must be between 5 to 255 characters"
    ))]
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Deserialize, Clone, ToSchema, Validate)]
pub struct UpdateTask {
    #[validate(length(
        min = 5,
        max = 255,
        message = "Title must be between 5 to 255 characters"
    ))]
    pub title: Option<String>,
    pub done: bool,
}

#[derive(Debug, Deserialize, Clone, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    pub id: i32,
    pub pid: String,
    pub user_pid: String,
    pub title: String,
    pub done: bool,
    pub created_at: String,
}

impl From<Task> for TaskResponse {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            pid: value.pid.to_string(),
            user_pid: value.user_pid.to_string(),
            title: value.title.to_string(),
            done: value.done,
            created_at: value.created_at.format("%d-%m-%Y %H:%M:%S").to_string(),
        }
    }
}
