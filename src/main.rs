use hyper::upgrade::Upgraded;
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use std::sync::Arc;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use rustls::{NoClientAuth, ServerConfig};
use std::net::SocketAddr;
use tokio_rustls::TlsAcceptor;

pub mod cert;

#[tokio::main]
async fn main() {
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&socket_addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.method() == "CONNECT" {
        tokio::task::spawn(async {
            let authority = req
                .uri()
                .authority()
                .expect("Cannot get authority of given url")
                .to_owned();

            let uri = http::uri::Builder::new()
                .scheme("https")
                .authority(authority)
                .path_and_query("/")
                .build()
                .expect("Couldnt build uri");

            dbg!(uri);

            match req.into_body().on_upgrade().await {
                Ok(upgraded) => {
                    if let Err(e) = tunnel(upgraded).await {
                        eprintln!("Error on tunneling {:?}", e)
                    }
                }
                Err(e) => eprintln!("Error on ugprading {:?}", e),
            }
        });

        return Ok(Response::new(Body::empty()));
    }

    Ok(Response::new(Body::from("ONLY HTTPS no HTTP")))
}

async fn tunnel(upgraded: Upgraded) -> std::io::Result<()> {
    let (key, crts) = cert::generate_cert(&http::uri::Authority::from_static("jsonip.com"));
    let mut config = ServerConfig::new(NoClientAuth::new());

    config.set_single_cert(crts, key).expect("Invalid Cert");

    let stm = TlsAcceptor::from(Arc::new(config)).accept(upgraded).await;

    if let Ok(stream) = stm {
        println!("HELL YEAH");
        //hyper::server::conn::Http::new().serve_connection(s, service)
    }
    Ok(())
}
