use actix_web::http::StatusCode;
use sea_orm::{ActiveModelTrait, IntoActiveModel, ModelTrait, QueryFilter, Set, TransactionTrait};
use actix_web::web;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait};
use sea_orm::prelude::DateTimeWithTimeZone;
use crate::models::dto_models::UserListDto;
use crate::models::response::ApiResponse;
use crate::models::user_models::{UserCreate, UserUpdate};

pub async fn get_users_list(
    db: web::Data<DatabaseConnection>,
) -> Result<ApiResponse<Vec<UserListDto>>, ApiResponse<String>> {

    let user_with_projects = entity::user::Entity::find()
        .find_also_related(entity::project::Entity)
        .all(db.as_ref())
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    let users: Vec<entity::user::Model> = user_with_projects.iter().map(|(model, _)| model.clone()).collect();

    let roles = users.load_many_to_many(
        entity::role::Entity,
        entity::user_role::Entity,
        db.as_ref()
    )
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    let mut user_dto = vec![];

    for( (user, project), roles) in user_with_projects.into_iter().zip(roles.into_iter()) {
        let role_name: Vec<String> = roles.into_iter().map(|r| r.role_name).collect();

        let project_name = match project {
            Some(p) => p.project_name,
            None => "".to_string()
        };

        user_dto.push(UserListDto {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            is_active: user.is_active,
            role_name,
            project_name,
        })
    }

    Ok(ApiResponse::success(200, user_dto))
}

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<UserCreate>
) -> Result<ApiResponse<String>, ApiResponse<String>> {

    let user_data = user_data.into_inner();

    let txn = db.begin()
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    let existing_user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(&user_data.email))
        .one(&txn)
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    let hash_password = hash(&user_data.password, DEFAULT_COST)
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    let new_user = match existing_user {
        Some(_) => {
            return Err(ApiResponse::error(StatusCode::CONFLICT.as_u16(), "User already exists".to_string()))
        }
        None => {
            entity::user::ActiveModel {
                first_name: Set(user_data.first_name),
                last_name: Set(user_data.last_name),
                email: Set(user_data.email),
                password: Set(hash_password),
                is_active: Set(user_data.is_active),
                project_id: Set(user_data.project_id),
                ..Default::default()
            }
        }
            .insert(&txn)
            .await
            .map_err(|err| ApiResponse::error(500, err.to_string()))
    };

    let user_role = entity::user_role::ActiveModel {
        role_id: Set(user_data.role_id),
        user_id: Set(new_user?.id),
    }.insert(&txn)
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    txn.commit()
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    Ok(ApiResponse::success(201, "User was created".to_string()))
}

pub async fn delete_user(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>
) -> Result<ApiResponse<String>, ApiResponse<String>> {

    let existing_user = entity::user::Entity::find()
        .filter(entity::user::Column::Id.eq(user_id.into_inner()))
        .one(db.as_ref())
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;
    
    match existing_user {
        Some(user) => {
            user.delete(db.as_ref()).await.map_err(|err| ApiResponse::error(500, err.to_string()))?;
        }
        None => {
            return Err(ApiResponse::error(StatusCode::NOT_FOUND.as_u16(), "User not found".to_string()))
        }
    }
    
    Ok(ApiResponse::success(200, "User was deleted".to_string()))
}

pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    user_data: web::Json<UserUpdate>
) -> Result<ApiResponse<String>, ApiResponse<String>> {
    
    let user_data = user_data.into_inner();

    let existing_user = entity::user::Entity::find()
        .filter(entity::user::Column::Id.eq(user_data.user_id))
        .one(db.as_ref())
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;
    
    match existing_user {
        Some(user) => {
            let mut active_model = user.into_active_model();
            active_model.updated_at = Set(DateTimeWithTimeZone::from(Utc::now()));
            if let Some(first_name) = user_data.first_name {
                active_model.first_name = Set(first_name);
            }
            if let Some(last_name) = user_data.last_name {
                active_model.last_name = Set(last_name);
            }
            active_model.email = Set(user_data.email);
            active_model.update(db.as_ref())
                .await
                .map_err(|err| ApiResponse::error(500, err.to_string()))?;
        }
        
        None => {
            return Err(ApiResponse::error(StatusCode::NOT_FOUND.as_u16(), "User not found".to_string()))
        }
    }
    
    Ok(ApiResponse::success(200, "User was updated".to_string()))
}