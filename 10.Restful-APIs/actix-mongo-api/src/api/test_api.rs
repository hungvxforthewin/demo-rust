use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
    Responder
};

#[get("/test")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from test APIs")
}