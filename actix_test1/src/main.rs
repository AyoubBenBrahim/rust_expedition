use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json::json;


#[get("/")]
async fn hello() -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    HttpResponse::Ok().body(
        format!("GET request recieved on route: /")
    )
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(
        format!("POST request recieved: {}", req_body)
    )
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// Define a handler for non-existing routes
async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "error": "Not Found",
        "message": "The requested resource was not found"
    }))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .default_service(web::route().to(not_found))

            
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3333))?
    .run()
    .await
}