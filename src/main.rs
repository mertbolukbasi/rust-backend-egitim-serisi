use actix_web::{App, HttpServer};
use sea_orm::Database;
use crate::config::config::AppConfig;

mod config;

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

	HttpServer::new(|| {
		App::new()
	})
		.bind(&url)?
		.run()
		.await

}
