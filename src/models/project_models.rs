use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProjectCreate {
	pub project_name: String,
	pub is_active: bool,
}

#[derive(Deserialize)]
pub struct ProjectDeleteQuery {
	pub project_id: u32,
}

#[derive(Deserialize)]
pub struct ProjectUpdate {
	pub project_id: u32,
	pub project_name: String,
	pub is_active: bool,
}