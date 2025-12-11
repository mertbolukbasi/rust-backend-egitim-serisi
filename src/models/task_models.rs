use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskCreate {
    pub task_name: String,
    pub user_id: i32,
    pub project_id: i32,
    pub due_time: String,
    pub is_active: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskUpdate {
    pub task_id: i32,
    pub due_time: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}