use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?.use_last_modified(true))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    let port = std::env::var("APP_PORT").expect("APP_PORT must be set");
    let bind = format!("0.0.0.0:{}", port);
    let ssl_mode = std::env::var("SSL_MODE").unwrap_or("false".to_string());

    if ssl_mode == "true" {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("certs/key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("certs/cert.pem").unwrap();

        HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
            .bind_openssl(&bind, builder)?
            .run()
            .await
    } else {
        HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
            .bind(&bind)?
            .run()
            .await
    }
}
