use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserCreate {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub project_id: i32,
    pub role_id: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserUpdate {
    pub user_id: i32,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    
    pub email: String,
}