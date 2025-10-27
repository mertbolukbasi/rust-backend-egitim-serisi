use serde::Deserialize;

#[derive(Deserialize)]
pub struct RoleCreate {
	pub role_name: String,
}

#[derive(Deserialize)]
pub struct RoleUpdate {
	pub role_id: u32,
	pub role_name: String,
}