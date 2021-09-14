#![allow(non_snake_case)]
mod db_layer;
mod ws;
use actix_web::*;
use mysql::*;

#[actix_rt::main]
async fn main() {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("4001".to_string());
    let address = format!("127.0.0.1:{}", port);
    let user_db = std::env::var("USER_DB").unwrap_or("mysql".to_string());
    let pass = std::env::var("PASS").unwrap_or("1234".to_string());
    let host = std::env::var("HOST").unwrap_or("localhost".to_string());
    let port_db = std::env::var("PORT_DB").unwrap_or("3306".to_string());
    let database = std::env::var("DATABASE").unwrap_or("mysql".to_string());
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user_db, pass, host, port_db, database
    );
    let opts = Opts::from_url(&url).unwrap();
    let pool = match Pool::new(opts) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to open DB connection. {:?}", e);
            return;
        }
    };
    let shared_data = web::Data::new(pool);
    let server = match HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(ws::get_publication_byParams)
            .service(ws::getPublication_byId)
            .service(ws::create_publication)
            .service(ws::saveR)
            .service(ws::get_All)
    })
    .bind(&address)
    {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind port. {:?}", e);
            return;
        }
    };
    match server.run().await {
        Ok(_) => println!("Server exited normally."),
        Err(e) => println!("Server exited with error: {:?}", e),
    };
}
