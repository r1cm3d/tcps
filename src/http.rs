use actix_web::{delete, get, put, web, HttpResponse, Responder};

use crate::tcp;

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/listeners")]
async fn get_listeners(data: web::Data<crate::AppData>) -> impl Responder {
    // TODO: Handle it properly later.
    let listeners = &data.listeners.lock().unwrap();
    HttpResponse::Ok().body(format!("Listeners: {:?}", listeners))
}

#[put("/listener/{port}")]
async fn listen(data: web::Data<crate::AppData>, path: web::Path<u16>) -> impl Responder {
    let port = path.into_inner();
    let tx = data.tx.clone();

    match tcp::bind(port, tx) {
        Ok(_) => HttpResponse::Ok().body(format!("Listening at {port}.")),
        Err(msg) => HttpResponse::InternalServerError().body(msg),
    }
}

#[delete("/listener/{port}")]
async fn stop(data: web::Data<crate::AppData>, path: web::Path<u16>) -> impl Responder {
    let port = path.into_inner();
    let tx = data.tx.clone();

    tcp::stop(port, tx).await;

    HttpResponse::Accepted().body(format!("Stopping to listen at {port}."))
}
