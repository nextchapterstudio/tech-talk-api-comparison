use actix_web::{web::{self}, App, HttpResponse, HttpServer, Responder};


use sqlx::{PgPool};

#[derive(serde::Serialize)]
struct Todo {
    id: i32,
    description: String,
    completed: bool,
}

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
            .route("/toggle-item/{id}", web::post().to(toggle_item))
            .route("/create-item", web::post().to(create_item))


    }).bind("127.0.0.1:8080")?
        .run()
        .await
}



async fn get_items(pool: web::Data<PgPool>) -> impl Responder {
    match  sqlx::query_as!(
        Todo,
        r#"
        SELECT * FROM todo
        "#
        )
        .fetch_all(pool.get_ref())
        .await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {println!("Error: {:?}", e); HttpResponse::InternalServerError().finish()}
        }
}

async fn toggle_item(path: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    match  sqlx::query_as!(
        Todo,
        r#"
        UPDATE todo SET completed = NOT completed
        WHERE id = $1
        RETURNING *
        "#,
        path.into_inner())
        .fetch_one(pool.get_ref())
        .await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {println!("Error: {:?}", e); HttpResponse::InternalServerError().finish()}
        }
}
    


#[derive(serde::Deserialize)]
struct CreateTodo {
    description: String,
}


async fn create_item(todo: web::Json<CreateTodo>, pool: web::Data<PgPool>) -> impl Responder {
   match  sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todo (description)
        VALUES ($1)
        RETURNING id, description, completed
        "#,
        todo.description)
        .fetch_one(pool.get_ref())
        .await {
            Ok(result) => HttpResponse::Ok().json(result),
            Err(e) => {println!("Error: {:?}", e); HttpResponse::InternalServerError().finish()}
        }
    
}