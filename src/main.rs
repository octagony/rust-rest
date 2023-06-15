use ntex::web;
use std::io;
mod services;

#[web::get("/")]
async fn index()-> &'static str{
    "Hello world!"
}
#[ntex::main]
async fn main() -> io::Result<()> {
    web::server(||{
        web::App::new().default_service(web::route().to(services::default))
    } )
    .bind(("0.0.0.0",8080))?
    .run()
    .await?;
    Ok(())
}
