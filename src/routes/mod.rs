pub mod roles;
pub mod projects;
pub mod users;
mod tasks;

use actix_web::web;

pub fn routes_init(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/api")
			.configure(roles::roles_init)
			.configure(projects::projects_init)
			.configure(users::users_init)
			.configure(tasks::tasks_init),
	);
}