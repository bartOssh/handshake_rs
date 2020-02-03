#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate actix_web;
extern crate simple_logger;

mod interfaces;
mod shared_utils;

use actix_web::{http, web, App, HttpResponse, HttpServer, Responder};
use interfaces::{request::RequestEvent, response::ResponseEvent};
use lambda::error::{HandlerError, LambdaErrorExt};
use shared_utils::errors::ErrorStr;
use std::env;
use std::error::Error;
use std::fmt;

const LOCAL_ADDRESS: &str = "0.0.0.0:3000";

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    if let Ok(lambda_env) = env::var("LAMBDA_ENV") {
        if lambda_env == format!("true") {
            lambda!(main_api_handler);
        } else {
            HttpServer::new(|| {
                App::new()
                    .route("/", web::post().to(local_responder))
                    .route(
                        "/",
                        web::method(http::Method::OPTIONS).to(options_responder),
                    )
            })
            .bind(LOCAL_ADDRESS)
            .unwrap()
            .run()
            .unwrap();
        }
    } else {
        return Err(String::from("Environment variable LAMBDA_ENV has not been set").into());
    }
    Ok(())
}

fn options_responder() -> impl Responder {
    return HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "X-PINGOTHER, Content-Type")
        .header("Access-Control-Max-Age", "86400")
        .status(http::StatusCode::from_u16(204).unwrap())
        .finish();
}

fn local_responder(e: web::Json<RequestEvent>) -> impl Responder {
    if e.day != "" {
        let day: u8 = e.day.parse().unwrap();
        let result = match day % 3 {
            0 => "handshake",
            1 => "bro-fist",
            2 => "glock",
            _ => unreachable!(),
        };
        return HttpResponse::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .json(ResponseEvent {
                handshake_type: result.to_string(),
            });
    }
    HttpResponse::MethodNotAllowed().body("405 Method Not Allowed")
}

fn main_api_handler(e: RequestEvent, c: lambda::Context) -> Result<String, HandlerError> {
    if e.day == "" {
        error!("Empty day in request {}", c.aws_request_id);
        return Err(HandlerError::new(ErrorStr {
            value: String::from("No day has been provided"),
        }));
    }
    // TODO: Here all api requests are going to be handled
    Ok(format!("simple message"))
}

#[cfg(test)]
mod tests_main {
    #[test]
    fn test_api_handler() {
        assert_eq!(2 + 2, 4);
    }
}
