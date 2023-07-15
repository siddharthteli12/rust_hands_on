use dotenv::dotenv;


#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let names = vec!["Sid".to_string(), "Akhil".to_string(), "Tanuj".to_string()];
    let ages: Vec<Option<i64>> = vec![Some(23), None, None];

    let url = std::env::var("DATABASE_URL").expect("ENV VARIABLE must be set.");

    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    // Comment query code below for table creation.
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    sqlx::query!(
        "INSERT INTO students(name ,age) 
        SELECT * FROM UNNEST($1::text[], $2::int8[])",
        &names[..],
        &ages[..]: Vec<Option<i64>>,
    )
    .execute(&pool)
    .await
    .unwrap();
}
