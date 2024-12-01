use actix_web::{web, HttpResponse, Responder};
use redis::aio::MultiplexedConnection;
use crate::username_and_password::UsernameAndPassword;

pub async fn add_user(user: web::Json<UsernameAndPassword>, redis_connection: web::Data<MultiplexedConnection>) -> impl Responder
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