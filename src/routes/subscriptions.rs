use actix_web::{web,HttpResponse};
use chrono::Utc;
use uuid::Uuid;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {

    match sqlx::query!(
        r#"
        INSERT INTO subscription (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#          ,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            println!("Failed to execute query {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}
