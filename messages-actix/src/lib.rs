#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpServer, Result};
use serde::Serialize;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct MessageApp {
    port: u16,
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    message: Vec<String>,
}

struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        message: ms.to_owned(),
    }))
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting http server 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}
