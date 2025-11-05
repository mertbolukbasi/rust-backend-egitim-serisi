use actix_web::web;
use crate::handler::project_handler;

pub fn projects_init(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/projects")
			.service(
				web::resource("/list")
					.route(web::get().to(project_handler::get_projects_list))
			)
			.service(
				web::resource("/create")
					.route(web::post().to(project_handler::create_project))
			)
			.service(
				web::resource("/delete")
					.route(web::delete().to(project_handler::delete_project))
			)
			.service(
				web::resource("/update")
					.route(web::put().to(project_handler::update_project))
			)
	);
}