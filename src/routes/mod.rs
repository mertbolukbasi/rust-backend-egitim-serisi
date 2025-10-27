pub mod roles;

use actix_web::web;

pub fn routes_init(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/api")
			.configure(roles::roles_init)
	);
}