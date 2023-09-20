use actix_web::{web::{self, Json}, App, HttpResponse, HttpRequest, HttpServer, Responder};


use sqlx::{PgPool};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let pool = PgPool::connect("postgres://postgres:changeme@localhost:5432/mydb")
        .await
        .expect( "Failed to connect to Postgres.");

    let connect = web::Data::new(pool);

    HttpServer::new(move || {
      App::new()
            .app_data(connect.clone())
            .route("/get-items", web::get().to(get_items))
            .route("/toggle-item", web::post().to(toggle_item))
            .route("/create-item", web::post().to(create_item))


    }).bind("127.0.0.1:8080")?
        .run()
        .await
}



async fn get_items(_connection: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn toggle_item(_connection: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct CreateTodo {
    description: String,
}

async fn create_item(todo: web::Json<CreateTodo>, pool: web::Data<PgPool>) -> impl Responder {
   match  sqlx::query!(
        r#"
        INSERT INTO todo (description)
        VALUES ($1)
        RETURNING *
        "#,
        todo.description)
        .execute(pool.get_ref())
        .await {
            Ok(result) => HttpResponse::Ok().json(format!("{} rows affected", result.rows_affected())),
            Err(e) => {println!("Error: {:?}", e); HttpResponse::InternalServerError().finish()}
        }
    
}