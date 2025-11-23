use actix_web::web;
use crate::handler::user_handler;

pub fn users_init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("/list")
                    .route(web::get().to(user_handler::get_users_list))
            )
            .service(
                web::resource("/create")
                    .route(web::post().to(user_handler::create_user))
            )
            .service(
                web::resource("/delete/{user_id}")
                    .route(web::delete().to(user_handler::delete_user))
            )
            .service(
                web::resource("/update")
                    .route(web::put().to(user_handler::update_user))
            )
    );
}