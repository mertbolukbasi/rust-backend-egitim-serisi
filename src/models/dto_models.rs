use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct UserListDto {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_active: bool,
    pub role_name: Vec<String>,
    pub project_name: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskListDto {
    pub task_name: String,
    pub user: entity::user::Model,
    pub project: entity::project::Model,
}