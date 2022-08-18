use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let subscriber_id = Uuid::new_v4();

    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        subscriber_id,
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .await
    .expect("Failed to insert subscription");
    HttpResponse::Ok().finish()
}
