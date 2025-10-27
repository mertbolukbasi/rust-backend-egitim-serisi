use actix_web::{web, App, HttpServer};
use sea_orm::Database;
use crate::config::config::AppConfig;

mod config;
mod models;
mod routes;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let cfg = AppConfig::from_env().expect("Failed to read configuration");
	let host = cfg.host;
	let port = cfg.port;
	let url = format!("{}:{}", host, port);
	let database_url = cfg.database_url;

	let connection = Database::connect(database_url)
		.await
		.expect("Failed to connect to database");

	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(connection.clone()))
			.configure(routes::routes_init)
	})
		.bind(&url)?
		.run()
		.await

}
