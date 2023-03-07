use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};

mod api;
mod models;
mod repository;

use api::users::{create_user, get_user};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use repository::ddb::DDBRepository;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;

    HttpServer::new(move || {
        let client = Client::new(&config);
        let ddb_repo: DDBRepository = DDBRepository::init(client.clone());
        let ddb_data = Data::new(ddb_repo);
        App::new()
            .app_data(ddb_data)
            .service(health)
            .service(get_user)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
