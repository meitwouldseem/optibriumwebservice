use actix_web::{web, HttpResponse, Responder};
use redis::aio::MultiplexedConnection;

pub async fn get_usernames(redis_connection: web::Data<MultiplexedConnection>) -> impl Responder
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
