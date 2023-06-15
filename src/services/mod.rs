use ntex::web;
pub mod dogs;

pub async fn default()->web::HttpResponse{
	web::HttpResponse::NotFound().finish()
}