use ntex::web;

#[web::get("/dogs")]
pub async fn get_dogs()->web::HttpResponse{
	web::HttpResponse::Ok().finish()
}

#[web::post("/dogs")]
pub async fn create_dog()->web::HttpResponse{
	web::HttpResponse::Created().finish()
}

#[web::get("/dogs/{id}")]
pub async fn get_dog()->web::HttpResponse{
	web::HttpResponse::Ok().finish()
}

#[web::put("/dogs/{id}")]
pub async fn update_dog()->web::HttpResponse{
	web::HttpResponse::Ok().finish()
}

#[web::delete("/dogs/{id}")]
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


