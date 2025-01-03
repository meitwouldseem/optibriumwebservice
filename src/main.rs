use redis::{aio::MultiplexedConnection, RedisResult};
use actix_web::{web, App, HttpServer};

mod routes;
mod username_and_password;

async fn make_redis_connection(address: &str) -> RedisResult<MultiplexedConnection>
{
    let client = redis::Client::open(address)?;
    client.get_multiplexed_tokio_connection().await
}

#[tokio::main]
async fn main() {
    let redis_connection = make_redis_connection("redis://127.0.0.1")
    .await
    .expect("Could not open Redis connection.");

    HttpServer::new(move || {
        App::new()
            .route("health_check", web::get().to(routes::health_check::health_check))
            .route("get_usernames", web::get().to(routes::get_usernames::get_usernames))
            .route("add_user", web::post().to(routes::add_user::add_user))
            .route("check_password", web::post().to(routes::check_password::check_password))
            .app_data(web::Data::new(redis_connection.clone()))
    })
    .bind("127.0.0.1:8000").expect("Could not bind server to address")
    .run()
    .await.expect("Creating HTTP server failed")
}
