use actix_web::{web::Form, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscribeBody {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<SubscribeBody>) -> impl Responder {
    println!("{:?} {:?}", form.email, form.name);
    HttpResponse::Ok()
}
