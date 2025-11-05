use actix_web::http::StatusCode;
use sea_orm::{ActiveModelTrait, IntoActiveModel, ModelTrait, QueryFilter, Set};
use sea_orm::ColumnTrait;
use actix_web::web;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::prelude::DateTimeWithTimeZone;
use entity::role::Model;
use crate::models::response::ApiResponse;
use crate::models::role_models::{RoleCreate, RoleUpdate};

pub async fn get_roles_list(
	db: web::Data<DatabaseConnection>
) -> Result<ApiResponse<Vec<Model>>, ApiResponse<String>> {

	let roles = entity::role::Entity::find()
		.all(db.as_ref())
		.await
		.map_err(|err| ApiResponse::error(500, err.to_string()))?;

	Ok(ApiResponse::success(200, roles))
}

pub async fn create_role(
	db: web::Data<DatabaseConnection>,
	role_data: web::Json<RoleCreate>
) -> Result<ApiResponse<String>, ApiResponse<String>> {

	let role_name = role_data.into_inner().role_name;

	let existing_role = entity::role::Entity::find()
		.filter(entity::role::Column::RoleName.eq(role_name.clone()))
		.one(db.as_ref())
		.await
		.map_err(|err| ApiResponse::error(500, format!("Role finding error: {}", err.to_string())))?;

	if existing_role.is_some() {
		return Err(ApiResponse::error(400, "Role already exists".to_string()));
	}

	let new_role = entity::role::ActiveModel {
		role_name: Set(role_name),
			..Default::default()
	}.save(db.as_ref()).await
		.map_err(|err| ApiResponse::error(500, format!("Unable to save role: {}", err.to_string())))?;

	Ok(ApiResponse::success(StatusCode::CREATED.as_u16(), "Role was created".to_string()))
}

pub async fn delete_role(
	db: web::Data<DatabaseConnection>,
	role_id: web::Path<u32> // http://127.0.0.1:8080/api/roles/delete/2
) -> Result<ApiResponse<String>, ApiResponse<String>> {
	
	let existing_role = entity::role::Entity::find()
		.filter(entity::role::Column::Id.eq(*role_id))
		.one(db.as_ref())
		.await
		.map_err(|err| ApiResponse::error(500, format!("Unable to find role: {}", err.to_string())))?;
	
	if existing_role.is_none() {
		return Err(ApiResponse::error(400, "Role does not exist".to_string()));
	}
	
	let deleted_role = existing_role.unwrap().delete(db.as_ref()).await
		.map_err(|err| ApiResponse::error(500, format!("Unable to delete role: {}", err.to_string())))?;
	
	Ok(ApiResponse::success(StatusCode::OK.as_u16(), "Role was deleted".to_string()))
}

pub async fn update_role(
	db: web::Data<DatabaseConnection>,
	role_data: web::Json<RoleUpdate>
) -> Result<ApiResponse<String>, ApiResponse<String>> {

	let existing_role = entity::role::Entity::find()
		.filter(entity::role::Column::Id.eq(role_data.role_id.clone()))
		.one(db.as_ref())
		.await
		.map_err(|err| ApiResponse::error(500, format!("Unable to find role: {}", err.to_string())))?;

	let role_model = match existing_role {
		Some(model) => model,
		None => return Err(ApiResponse::error(400, "Role does not exist".to_string()))
	};

	let mut active_role = role_model.into_active_model();
	active_role.role_name = Set(role_data.role_name.clone());
	active_role.updated_at = Set(DateTimeWithTimeZone::from(Utc::now()));
	active_role.save(db.as_ref()).await
		.map_err(|err| ApiResponse::error(500, format!("Unable to save role: {}", err.to_string())))?;

	Ok(ApiResponse::success(StatusCode::OK.as_u16(), "Role was updated".to_string()))
}