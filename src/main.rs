use fastly::http::{header, Method, StatusCode};
use fastly::{Error, Request, Response};

pub mod codec;
use crate::codec::{decode_body, encode_body};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // serve the demo client app
    if req.get_method() == Method::GET {
        match req.get_path() {
            "/" => {
                let mut resp = Response::from_status(StatusCode::OK);
                resp.set_header(header::CONTENT_TYPE, "text/html");
                resp.set_body(include_str!("../client/index.html"));
                return Ok(resp);
            }
            "/index.js" => {
                let mut resp = Response::from_status(StatusCode::OK);
                resp.set_header(header::CONTENT_TYPE, "text/javascript");
                resp.set_body(include_str!("../client/dist/main.js"));
                return Ok(resp);
            }
            _ => {}
        }
    }

    // assert content type
    if req
        .get_header(header::CONTENT_TYPE)
        .map(|h| h.to_str().unwrap())
        != Some("application/grpc-web+proto")
    {
        let mut resp = Response::from_status(StatusCode::BAD_REQUEST);
        resp.set_header(header::CONTENT_TYPE, "text/plain");
        resp.set_body("invalid content type");
        return Ok(resp);
    }

    // extract service and method
    let path = req.get_path().to_string();
    let (service, method) = path[1..].split_once('/').unwrap();
    assert!(!service.contains('/'));
    assert!(!method.contains('/'));

    println!("received RPC request for {service}/{method}");

    let mut resp = Response::from_status(StatusCode::OK);
    resp.set_header(header::CONTENT_TYPE, "application/grpc-web+proto");

    // match on service/method, build a response
    resp.set_body(encode_body(match (service, method) {
        ("helloworld.Greeter", "SayHello") => {
            let client_message = decode_body::<hello_world::HelloRequest>(req.into_body());

            hello_world::HelloReply {
                message: format!("Hello, {}!", client_message.name),
            }
        }
        _ => {
            panic!("unsupported method")
        }
    }));

    Ok(resp)
}
