use crate::parser::{prometheus, Metric};
use hyper::{
    Body,
    Method,
    Request,
    Response,
    Server,
    StatusCode,
    service::{make_service_fn, service_fn},
};
use snap::raw::Decoder;
use std::{io, convert::Infallible, net::ToSocketAddrs};
use tokio::{self, sync::broadcast::Sender};

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

async fn http_server(sender: Sender<Metric>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut decoder = Decoder::new();
    match req.method() {
        &Method::GET => Ok(
            Response::new(
                Body::from("Syphon Prometheus Remote Write Interface
see prometheus docs: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#remote_write")
            )
        ),

        &Method::POST => {
            // Decode body, parse prometheus metric in to graphite, Send to channel
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let decoded = decoder
                .decompress_vec(&whole_body)
                .expect("unable to decode prometheus");
            for metric in prometheus::parse(decoded) {
                sender
                    .send(metric)
                    .expect("failed to write data to channel");
            }
            Ok(Response::new(Body::from("Message Recieved")))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
