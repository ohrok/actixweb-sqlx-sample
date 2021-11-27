use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://actix_username:actix_password@localhost:5432/actix_database")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}
