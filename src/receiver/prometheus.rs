use std::{io, convert::Infallible, net::ToSocketAddrs};
use snap::raw::Decoder;
use crate::parser::Metric;
use tokio::{self, sync::broadcast::Sender};
use hyper::{
    Body,
    Method,
    Request,
    Response,
    Server,
    StatusCode,
    service::{make_service_fn, service_fn},
};

pub async fn bind(addr: String, sender: Sender<Metric>) -> Result<(), io::Error> {
    let mut address = addr.to_socket_addrs().unwrap();
    let listener = Server::bind(&address.next().unwrap())
        .serve(
            make_service_fn(move |_| {
                let sender = sender.clone();
                async move {
                    Ok::<_, Infallible>(
                        service_fn(move |req| http_server(sender.clone(), req))
                    )
                }
            })
        );
    info!(
        "Reciever listening; proto={} addr={}",
        "prometheus",
        listener.local_addr()
    );
    if let Err(err) = listener.await {
        error!("prometheus server error; err={}", err);
    }
    Ok(())
}

async fn http_server(_sender: Sender<Metric>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let _decoder = Decoder::new();
    match req.method() {
        &Method::GET => Ok(
            Response::new(
                Body::from("Syphon Prometheus Remote Write Interface
see prometheus docs: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#remote_write")
            )
        ),

        &Method::POST => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
