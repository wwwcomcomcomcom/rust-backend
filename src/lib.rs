use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req:HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    //HttpResponse::Ok().finish()
    HttpResponse::Ok()
    // HttpResponse::Ok().body("Server is alive!")
}

pub fn run(listener:TcpListener) -> Result<Server,std::io::Error>{
    println!("Server started");
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}