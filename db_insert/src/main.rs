use dotenv::dotenv;

pub struct Student {
    name: String,
    age: Option<i32>,
}

impl Student {
    fn new(name: String, age: i32) -> Self {
        Self {
            name,
            age: Some(age),
        }
    }
}

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let students = vec![
        Student::new("Sid".to_string(), 23),
        Student::new("Akhil".to_string(), 24),
        Student::new("Tanuj".to_string(), 23),
    ];

    let names = vec!["Sid".to_string(), "Akhil".to_string(), "Tanuj".to_string()];
    let ages = vec![Some(23), Some(24), Some(25)];

    let url = std::env::var("DATABASE_URL").expect("ENV VARIABLE must be set.");

    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    // Comment query code below for table creation.
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    sqlx::query!(
        "INSERT INTO students(name ,age) 
        SELECT * FROM UNNEST($1::text[], $2::int8[])",
        &names[..],
        &ages[..]
    )
    .execute(&pool)
    .await
    .unwrap();
}
