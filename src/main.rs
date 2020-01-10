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

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use interfaces::{request::RequestEvent, response::ResponseEvent};
use lambda::error::{HandlerError, LambdaErrorExt};
use shared_utils::errors::ErrorStr;
use std::env;
use std::error::Error;
use std::fmt;

const LOCAL_ADDRESS: &str = "127.0.0.1:3000";

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    if let Ok(lambda_env) = env::var("LAMBDA_ENV") {
        if lambda_env == format!("true") {
            lambda!(main_api_handler);
        } else {
            HttpServer::new(|| App::new().route("/", web::post().to(local_responder)))
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

fn local_responder(e: web::Json<RequestEvent>) -> impl Responder {
    if e.client_id != "" {
        return HttpResponse::Ok().json(ResponseEvent {
            value: format!("this is value"),
        });
    }
    HttpResponse::MethodNotAllowed().body("405 Method Not Allowed")
}

fn main_api_handler(e: RequestEvent, c: lambda::Context) -> Result<String, HandlerError> {
    if e.client_id == "" {
        error!("Empty client id in request {}", c.aws_request_id);
        return Err(HandlerError::new(ErrorStr {
            value: String::from("No client id has been provided"),
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
