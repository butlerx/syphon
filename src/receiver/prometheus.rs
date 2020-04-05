use async_trait::async_trait;
use std::{
    io,
    future,
    convert::Infallible,
    net::ToSocketAddrs
};
use super::Receiver;
use snap::raw::Decoder;
use tokio::{self, sync::broadcast::Sender};
use hyper::{
    Body,
    Method,
    Request,
    Response,
    Server as Hyper,
    StatusCode,
    service,
    server::conn::AddrIncoming
};

pub struct Server {
    listener: Hyper<
        AddrIncoming,
        dyn service::MakeServiceRef<
            dyn future::Future,
            Request<Body>
        >
    >,
}

#[async_trait]
impl Receiver for Server {
    async fn bind(addr: &String, sender: Sender<String>) -> io::Result<Server> {
        let address = &addr.to_socket_addrs().unwrap();
        let make_service = service::make_service_fn(move |_| {
            let sender = sender.clone();
            async move {
                Ok::<_, Infallible>(service::service_fn(
                    move |req| http_server(sender.clone(), req)
                ))
            }
        });
        let listener = Hyper::bind(&address.next().unwrap()).serve(make_service);
        Ok(Server{ listener })
    }

    fn addr(&self) -> io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }

    async fn run(&mut self) -> Result<(), io::Error> {
        self.listener.await;
        Ok(())
    }
}

async fn http_server(sender: Sender<String>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let decoder = Decoder::new();
    match req.method() {
        &Method::GET => Ok(Response::new(Body::from("Syphon Prometheus Remote Write Interface"))),

        &Method::POST => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let message = decoder.decompress_vec(&whole_body);

            Ok(Response::new(Body::from("Message Recieved")))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
