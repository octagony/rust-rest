use ntex::web;
use crate::models::Dog;

#[utoipa::path(
    get,
    path = "/dogs",
    responses(
        (status = 200, description = "List of dogs", body = [Dog]),
        ),
    )]
#[web::get("/api/dogs")]
pub async fn get_dogs()->web::HttpResponse{
	web::HttpResponse::Ok().finish()
}

#[utoipa::path(
    post,
    path = "/dogs",
    responses(
        (status = 201, description = "Dog created", body = Dog),
        ),
    )]
#[web::post("/api/dogs")]
pub async fn create_dog(_dog: web::types::Json<Dog>)->web::HttpResponse{
	web::HttpResponse::Created().finish()
}

#[web::get("/api/dogs/{id}")]
pub async fn get_dog()->web::HttpResponse{
	web::HttpResponse::Ok().finish()
}

#[web::put("/api/dogs/{id}")]
pub async fn update_dog()->web::HttpResponse{
	web::HttpResponse::Ok().finish()
}

#[web::delete("/api/dogs/{id}")]
pub async fn delete_dog()->web::HttpResponse{
	web::HttpResponse::Ok().finish()
}

pub fn ntex_config(cfg:&mut web::ServiceConfig){
	cfg.service(get_dogs);
	cfg.service(create_dog);
	cfg.service(get_dog);
	cfg.service(update_dog);
	cfg.service(delete_dog);
}


