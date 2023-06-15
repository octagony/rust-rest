use ntex::web;
use std::io;
// mods
mod services;
mod errors;
mod models;

#[web::get("/api")]
async fn index()-> &'static str{
    "Hello world!"
}
#[ntex::main]
async fn main() -> io::Result<()> {
    web::server(||{
        web::App::new()
        .configure(services::dogs::ntex_config)
        .default_service(web::route().to(services::default))
    } )
    .bind(("0.0.0.0",8080))?
    .run()
    .await?;
    Ok(())
}
