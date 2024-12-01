use std::vec::Vec;

use redis::{aio::MultiplexedConnection, RedisResult};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn make_redis_connection(address: &str) -> RedisResult<MultiplexedConnection>
{
    let client = redis::Client::open(address)?;
    client.get_multiplexed_tokio_connection().await
}

#[derive(serde::Deserialize)]
struct UsernameAndPassword
{
    username: String,
    password: String
}

async fn add_user(user: web::Json<UsernameAndPassword>, redis_connection: web::Data<MultiplexedConnection>) -> impl Responder
{
    if user.username.len() > 20 || user.password.len() > 20
    {
        return HttpResponse::BadRequest();
    }

    let mut con = redis_connection.get_ref().clone();
    let set_result = redis::cmd("SET")
    .arg(&user.username)
    .arg(&user.password)
    .exec_async(&mut con).await;

    if set_result.is_err()
    {
        return HttpResponse::InternalServerError();
    }

    HttpResponse::Ok()
}

async fn check_password(user: web::Json<UsernameAndPassword>, redis_connection: web::Data<MultiplexedConnection>) -> impl Responder
{
    let mut con = redis_connection.get_ref().clone();
    let get_result: Result<String, redis::RedisError> = redis::cmd("GET")
    .arg(&user.username)
    .query_async(&mut con).await;

    let password = match get_result
    {
        Ok(password) => password,
        Err(_) => return HttpResponse::BadRequest()
    };

    if password != user.password
    {
        return HttpResponse::Forbidden();
    }

    HttpResponse::Ok()
}

async fn get_usernames(redis_connection: web::Data<MultiplexedConnection>) -> impl Responder
{
    let mut con = redis_connection.get_ref().clone();
    let get_result: Result<Vec<String>, redis::RedisError> = redis::cmd("KEYS")
    .arg("*")
    .query_async(&mut con).await;

    let names = match get_result
    {
        Ok(names) => names,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(names)
}

#[tokio::main]
async fn main() {
    let redis_connection = make_redis_connection("redis://127.0.0.1")
    .await
    .expect("Could not open Redis connection.");

    HttpServer::new(move || {
        App::new()
            .route("health_check", web::get().to(health_check))
            .route("get_usernames", web::get().to(get_usernames))
            .route("add_user", web::post().to(add_user))
            .route("check_password", web::post().to(check_password))
            .route("get_usernames", web::post().to(get_usernames))
            .app_data(web::Data::new(redis_connection.clone()))
    })
    .bind("127.0.0.1:8000").expect("Could not bind server to address")
    .run()
    .await.expect("Creating HTTP server failed")
}
