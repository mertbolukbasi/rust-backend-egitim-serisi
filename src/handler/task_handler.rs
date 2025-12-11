use sea_orm::{ActiveModelTrait, ColumnTrait, IntoActiveModel, ModelTrait, Set};
use actix_web::web;
use chrono::{NaiveDate, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::Pool;
use entity::prelude::Task;
use crate::models::dto_models::TaskListDto;
use crate::models::response::ApiResponse;
use crate::models::task_models::{TaskCreate, TaskUpdate};

pub async fn get_task_list(
    db: web::Data<DatabaseConnection>,
) -> Result<ApiResponse<Vec<TaskListDto>>, ApiResponse<String>> {

    let task_project_user = entity::task::Entity::find()
        .find_also_related(entity::project::Entity)
        .find_also_related(entity::user::Entity)
        .all(db.as_ref())
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    let mut task_dto: Vec<TaskListDto> = vec![];

    for(task, project, user) in task_project_user.into_iter() {
        task_dto.push(TaskListDto {
            task_name: task.task_name,
            user: user.unwrap(),
            project: project.unwrap(),
        })
    };

    Ok(ApiResponse::success(200, task_dto))
}

pub async fn create_task(
    db: web::Data<DatabaseConnection>,
    task_data: web::Json<TaskCreate>,
) -> Result<ApiResponse<String>, ApiResponse<String>> {
    let task = task_data.into_inner();

    let existing_task = entity::task::Entity::find()
        .filter(entity::task::Column::TaskName.eq(&task.task_name))
        .one(db.as_ref())
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    match existing_task {
        Some(_) => return Err(ApiResponse::error(400, "Task with that name already exists".to_string())),

        None => {
            let date = NaiveDate::parse_from_str(&task.due_time, "%d-%m-%Y")
                .map_err(|err| ApiResponse::error(500, err.to_string()))?
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc();

            entity::task::ActiveModel {
                task_name: Set(task.task_name),
                is_active: Set(task.is_active),
                project_id: Set(task.project_id),
                due_time: Set(task.due_time),
                user_id: Set(task.user_id),
                ..Default::default()
            }.save(db.as_ref())
                .await
                .map_err(|err| ApiResponse::error(500, err.to_string()))?;
        }
    }

    Ok(ApiResponse::success(201, "Task created".to_string()))
}

pub async fn delete_task(
    db: web::Data<DatabaseConnection>,
    task_id: web::Path<i32>,
) -> Result<ApiResponse<String>, ApiResponse<String>> {

    let existing_task = entity::task::Entity::find()
        .filter(entity::task::Column::Id.eq(task_id.into_inner()))
        .one(db.as_ref())
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    match existing_task {
        Some(task) => {
            task.delete(db.as_ref()).await.map_err(|err| ApiResponse::error(500, err.to_string()))?;
        }
        None => {
            return Err(ApiResponse::error(404, "Task not found".to_string()))
        }
    }

    Ok(ApiResponse::success(200, "Task deleted".to_string()))
}

pub async fn update_task(
    db: web::Data<DatabaseConnection>,
    task_data: web::Json<TaskUpdate>,
) -> Result<ApiResponse<String>, ApiResponse<String>> {
    let task = task_data.into_inner();

    let existing_task = entity::task::Entity::find()
        .filter(entity::task::Column::Id.eq(task.task_id))
        .one(db.as_ref())
        .await
        .map_err(|err| ApiResponse::error(500, err.to_string()))?;

    match existing_task {
        Some(taskModel) => {
            let date = NaiveDate::parse_from_str(&*task.due_time, "%d-%m-%Y")
                .map_err(|err| ApiResponse::error(500, err.to_string()))?
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc();

            let mut active_model = taskModel.into_active_model();
            active_model.updated_at = Set(DateTimeWithTimeZone::from(Utc::now()));
            if let Some(task_name) = task.task_name {
                active_model.task_name = Set(task_name);
            }
            if let Some(user_id) = task.user_id {
                active_model.user_id = Set(user_id);
            }
            if let Some(is_active) = task.is_active {
                active_model.is_active = Set(is_active);
            }
            active_model.due_time = Set(task.due_time);
            active_model.update(db.as_ref())
                .await
                .map_err(|err| ApiResponse::error(500, err.to_string()))?;
        }
        None => {
            return Err(ApiResponse::error(404, "Task not found".to_string()))
        }
    }

    Ok(ApiResponse::success(200, "Task updated".to_string()))
}