use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn start_connection() -> Pool<Postgres> {
    let pg_variables = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_variables)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("./src/db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}
