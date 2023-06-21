use ntex::web;
pub mod dogs;
pub mod openapi;

pub async fn default()->web::HttpResponse{
	web::HttpResponse::NotFound().finish()
}