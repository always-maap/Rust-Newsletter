use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let addr = listener.local_addr()?;

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    println!("Listening on http://{}", addr);

    Ok(server)
}
