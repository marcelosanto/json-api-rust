use actix_web::*;

mod routes;
use routes::catalogo::*;
use routes::info::*;
use routes::ping::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/info", web::get().to(info))
            .route("/catalogo", web::get().to(catalogo))
    });

    let porta = 9091;
    let api = api
        .bind(format!("127.0.0.1:{}", porta))
        .expect("Não conseguiu conectar");

    println!("Conectado com sucesso! {}", porta);

    api.run().await
}
