use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?.use_last_modified(true))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    let port = std::env::var("APP_PORT").expect("APP_PORT must be set");
    let bind = format!("0.0.0.0:{}", port);

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .bind(&bind)?
        .run()
        .await
}
