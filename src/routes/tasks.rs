use actix_web::web;
use crate::handler::task_handler;

pub fn tasks_init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .service(
                web::resource("/list")
                    .route(web::get().to(task_handler::get_task_list))
            )
            .service(
                web::resource("/create")
                    .route(web::post().to(task_handler::create_task))
            )
            .service(
                web::resource("/delete/{task_id}")
                    .route(web::delete().to(task_handler::delete_task))
            )
            .service(
                web::resource("/update")
                    .route(web::put().to(task_handler::update_task))
            )
    );
}