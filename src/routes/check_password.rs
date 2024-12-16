use actix_web::{web, HttpResponse, Responder};
use redis::aio::MultiplexedConnection;
use crate::username_and_password::UsernameAndPassword;

pub async fn check_password(user: web::Json<UsernameAndPassword>, redis_connection: web::Data<MultiplexedConnection>) -> impl Responder
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
