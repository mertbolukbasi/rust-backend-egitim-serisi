use std::env;

pub struct AppConfig {
	pub host: String,
	pub port: String,
	pub database_url: String,
}

impl AppConfig {
	pub fn from_env() -> Result<Self, env::VarError> {
		dotenv::dotenv().ok();
		let host = env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
		let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
		Ok(Self {
			host,
			port,
			database_url,
		})
	}

}