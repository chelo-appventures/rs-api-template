use actix_web::{get, web, App, HttpServer, Result, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Deserialize, Serialize, Clone)]
struct Product {
    id: u32,
    name: String,
    sku: String,
    amount: u32,
    price: f32
}

struct AppState {
    products: Mutex<Vec<Product>>
}

#[get("/api/{entity}/{id}")]
async fn getOne(path: web::Path<(String, String)>, state: web::Data<AppState>) -> Result<impl Responder> {
    let (entity, id) = path.into_inner();
    println!("{} {}", entity, id);
    let products = state.products.lock().unwrap();
    let product = products[0];
    Ok(web::Json(product))
}

/*
#[get("/api/{entity}")]
async fn listing(entity: web::Path<String>, state: web::Data<AppState>) -> Result<impl Responder> {
    let mut products = state.products.lock().unwrap();
    *products = *products.clone();
    Ok(web::Json(products))
}
*/

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let appState = AppState {
        products: Mutex::new(vec![
            Product { 
                id: 1, 
                name: String::from("p1"), 
                sku: String::from("123"), 
                amount: 2, price: 10.0
            },
            Product { 
                id: 1, 
                name: String::from("p2"), 
                sku: String::from("125"), 
                amount: 2, price: 13.0
            },
        ])
    };
    // Note: web::Data created _outside_ HttpServer::new closure
    let globalState = web::Data::new(appState);

    HttpServer::new(|| {
        App::new()
            .app_data(globalState.clone())
            .service(getOne)
            //.service(listing)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
