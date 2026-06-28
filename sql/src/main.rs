#[tokio::main]
async  fn main() {
    println!("=== Rust SQL ===");
    println!("在这里练习：SQl");

    sql_text().await;
}
use sqlx::PgPool;

async fn sql_text() {

    let pool = PgPool::connect(
        "postgres://postgres:123456@localhost/rust_demo"
    ).await.unwrap();


    let users = sqlx::query!(
        "select id, name from users"
    )
    .fetch_all(&pool)
    .await
    .unwrap();


    for user in users {
        println!("{} {}", user.id, user.name);
    }
}