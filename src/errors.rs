use ntex::web;
use ntex::http;
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result};
use std::error::Error;

#[derive(Clone,Debug,Serialize, Deserialize, ToSchema)]
pub struct HttpError{
	pub msg:String,
	#[serde(skip)]
	pub status: http::StatusCode,
}

impl Display for HttpError{
	fn fmt(&self, f:&mut Formatter<'_>)-> Result{
		write!(f,"[{}] {}", self.status, self.msg)
	}
}

impl Error for HttpError{}

impl web::WebResponseError for HttpError{
	fn error_response(&self, _:&web::HttpRequest)->web::HttpResponse{
		web::HttpResponse::build(self.status).json(&self)
	}
}

