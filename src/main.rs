use std::io;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("[Alice, Bob]")
}

async fn put_users() -> impl Responder {
    // here do some logic to put a new user ??????
    HttpResponse::Ok().body("success")
}

async fn say_hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let resp = format!("hello {}", name);
    HttpResponse::Ok().body(resp)
}

async fn render_tmpl(tera: web::Data<Tera>, req:HttpRequest) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Hacker Clone");
    data.insert("name", "Ni");

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> io::Result<()>{
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .data(tera)
            .service(
                web::resource("/")
                    .route(web::get().to(index))
            )
            .service(
                web::resource("/users")
                    .route(web::get().to(get_users))
                    .route(web::put().to(put_users))  
            )
            .service(
                web::resource("/hello/{name}")
                    .route(web::get().to(say_hello))
            )
            .service(
                web::resource("/tmpl/{name}")
                    .route(web::get().to(render_tmpl))
            )
    })
    .bind(("52.79.48.244/", 80))? 
    .run()
    .await
}