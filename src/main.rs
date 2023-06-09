#[macro_use]
extern crate log;

mod http;
mod tcp;

use actix_web::{middleware::Logger, web, App, HttpServer};
use bus::Bus;
use crossbeam_channel::{unbounded, Sender};
use env_logger::Env;
use std::sync::{Arc, Mutex};

// TODO:
// - Change the API. From /servers to /listeners;
// - Sockets can be: /connections or /sockets;

#[derive(Clone)]
struct AppData {
    listeners: Arc<Mutex<Vec<u16>>>,
    tx: Sender<tcp::Command>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let (tx, rx) = unbounded();
    // TODO: Change it, bus capacity must be a variable.
    let bus_close = Arc::new(Mutex::new(Bus::<u16>::new(100)));
    let listeners = Arc::new(Mutex::new(Vec::new()));
    info!("Bootstrapping the application");

    tcp::bootstrap(rx, bus_close.clone(), listeners.clone());

    let app_data = AppData {
        listeners,
        tx: tx.clone(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(http::status)
            .service(http::get_listeners)
            .service(http::listen)
            .service(http::stop)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
