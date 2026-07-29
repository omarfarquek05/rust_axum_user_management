pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL সেট করা হয়নি"),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET সেট করা হয়নি"),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT একটা সংখ্যা হতে হবে"),
        }
    }
}