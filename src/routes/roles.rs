use actix_web::web;
use crate::handler::role_handler;

pub fn roles_init(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/roles")
			.service(
				web::resource("/list")
					.route(web::get().to(role_handler::get_roles_list))
			)
			.service(
				web::resource("/create")
					.route(web::post().to(role_handler::create_role))
			)
			.service(
				web::resource("/delete/{role_id}")
					.route(web::delete().to(role_handler::delete_role))
			)
			.service(
				web::resource("/update")
					.route(web::put().to(role_handler::update_role))
			)
	);
}