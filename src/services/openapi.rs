use std::sync::Arc;

use ntex::web;
use ntex::http;
use ntex::util::Bytes;
use utoipa::OpenApi;

use crate::errors::HttpError;
use crate::models::{Dog};

use super::dogs;

#[derive(OpenApi)]
#[openapi(
	paths(
		dogs::get_dogs,
		dogs::create_dog,
		dogs::get_dog,
		dogs::update_dog,
		dogs::delete_dog
	),
	components(schemas(Dog, HttpError))
)]

pub (crate) struct ApiDoc;

#[web::get("/{tail}*")]
async fn get_swagger(
	tail:web::types::Path<String>,
	openapi_conf: web::types::State<Arc<utoipa_swagger_ui::Config<'static>>>
)-> Result <web::HttpResponse, HttpError>{
	if tail.as_ref() == "swagger.json"{
		let spec = ApiDoc::openapi().to_json().map_err(|err| HttpError {
			status: http::StatusCode::INTERNAL_SERVER_ERROR,
			msg:format!("Error generating OpenApi spec: {}", err),
		})?;
		return Ok(
			web::HttpResponse::Ok().content_type("application_json").body(spec)
		);
	}

	let conf = openapi_conf.as_ref().clone();
	match utoipa_swagger_ui::serve(&tail, conf.into()).map_err(|err| {
		HttpError {
			status: http::StatusCode::INTERNAL_SERVER_ERROR,
			msg:format!("Error serving Swagger UI: {}", err)
		}
	})? {
		None => Err(HttpError {
			status: http::StatusCode::NOT_FOUND,
			msg:format!("Path not found: {}", tail)
		}),
		Some(file)=>Ok({
			let bytes = Bytes::from(file.bytes.to_vec());
			web::HttpResponse::Ok().content_type(file.content_type).body(bytes)
		}),
	}
}

pub fn ntex_config(config: &mut web::ServiceConfig){
	let swagger_config = Arc::new(
		utoipa_swagger_ui::Config::new(["/explorer/swagger.json"]).use_base_layout(),
	);
	config.service(
		web::scope("/explorer/").state(swagger_config).service(get_swagger)
	);
}