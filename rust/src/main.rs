use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer, Responder};

#[derive(serde::Serialize, Clone)]
struct TodoItem {
    id: u8,
    name: String,
    completed: bool,
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut todo_list:web::Data<Vec<TodoItem>> = web::Data::new( vec![TodoItem {
        id: 1,
        name: "Learn Rust".to_string(),
        completed: false,
}]);
    let counter = web::Data::new(0u8);



    HttpServer::new(move || {
      App::new()
        .app_data(todo_list.clone())
        .app_data(counter.clone())
            .route("/", web::get().to(greet))


    }).bind("127.0.0.1:8080")?
        .run()
        .await
}


async fn greet(todo_items:web::Data<Vec<TodoItem>>) -> impl Responder {
    HttpResponse::Ok().json(todo_items)
}
