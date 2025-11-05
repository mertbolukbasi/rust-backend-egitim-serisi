use sea_orm::{ActiveModelTrait, ColumnTrait, IntoActiveModel, ModelTrait, Set};
use sea_orm::QueryFilter;
use actix_web::http::StatusCode;
use actix_web::web;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::prelude::DateTimeWithTimeZone;
use entity::project::Model;
use crate::models::project_models::{ProjectCreate, ProjectDeleteQuery, ProjectUpdate};
use crate::models::response::ApiResponse;
use crate::models::role_models::RoleUpdate;

pub async fn get_projects_list(
	db: web::Data<DatabaseConnection>
) -> Result<ApiResponse<Vec<Model>>, ApiResponse<String>> {

	let roles = entity::project::Entity::find()
		.all(db.as_ref())
		.await
		.map_err(|err| ApiResponse::error(500, err.to_string()))?;

	Ok(ApiResponse::success(200, roles))
}

pub async fn create_project(
	db: web::Data<DatabaseConnection>,
	project_data: web::Json<ProjectCreate>
) -> Result<ApiResponse<String>, ApiResponse<String>> {

	let data = project_data.into_inner();
	let project_name = data.project_name;
	let is_active = data.is_active;

	let existing_project = entity::project::Entity::find()
		.filter(entity::project::Column::ProjectName.eq(&project_name))
		.one(db.as_ref())
		.await
		.map_err(|err| ApiResponse::error(500, format!("Project finding error: {}", err.to_string())))?;

	if existing_project.is_some() {
		return Err(ApiResponse::error(400, "Project already exists".to_string()));
	}

	let new_project = entity::project::ActiveModel {
		project_name: Set(project_name),
		is_active: Set(is_active),
		..Default::default()
	}.save(db.as_ref()).await
		.map_err(|err| ApiResponse::error(500, format!("Unable to save project: {}", err.to_string())))?;


	Ok(ApiResponse::success(StatusCode::CREATED.as_u16(), "Role was created".to_string()))
}

pub async fn delete_project(
	db: web::Data<DatabaseConnection>,
	project_id: web::Query<ProjectDeleteQuery>
) -> Result<ApiResponse<String>, ApiResponse<String>> {

	let existing_project = entity::project::Entity::find()
		.filter(entity::project::Column::Id.eq(project_id.into_inner().project_id))
		.one(db.as_ref())
		.await
		.map_err(|err| ApiResponse::error(500, format!("Project finding error: {}", err.to_string())))?;

	if existing_project.is_none() {
		return Err(ApiResponse::error(400, "Project was not found".to_string()));
	}

	let deleted_project = existing_project.unwrap().delete(db.as_ref()).await
		.map_err(|err| ApiResponse::error(500, format!("Unable to delete project: {}", err.to_string())))?;
	
	Ok(ApiResponse::success(StatusCode::OK.as_u16(), "Role was deleted".to_string()))
}


pub async fn update_project(
	db: web::Data<DatabaseConnection>,
	project_data: web::Json<ProjectUpdate>
) -> Result<ApiResponse<String>, ApiResponse<String>> {

	let existing_project = entity::project::Entity::find()
		.filter(entity::project::Column::Id.eq(project_data.project_id.clone()))
		.one(db.as_ref())
		.await
		.map_err(|err| ApiResponse::error(500, format!("Unable to find project: {}", err.to_string())))?;

	let project_model = match existing_project {
		Some(model) => model,
		None => return Err(ApiResponse::error(400, "Project does not exist".to_string()))
	};

	let mut active_project = project_model.into_active_model();
	active_project.project_name = Set(project_data.project_name.clone());
	active_project.is_active = Set(project_data.is_active);
	active_project.updated_at = Set(DateTimeWithTimeZone::from(Utc::now()));
	active_project.save(db.as_ref()).await
		.map_err(|err| ApiResponse::error(500, format!("Unable to save project: {}", err.to_string())))?;

	Ok(ApiResponse::success(StatusCode::OK.as_u16(), "Project was updated".to_string()))
}