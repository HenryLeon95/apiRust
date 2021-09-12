#![allow(non_snake_case)]
mod db_layer;
use mysql::*;

fn main() {
    dotenv::dotenv().ok();
    //let port = std::env::var("PORT").unwrap_or("4001".to_string());
    //let address = format!("127.0.0.1:{}", port);
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
    if let Ok(pool) = Pool::new(opts) {
        if let Ok(mut conn) = pool.get_conn() {
            let publication = db_layer::Publication {
                Id: 0,
                Name: String::from("Rust 2"),
                Comment: String::from("Comentario medio de Rust 2"),
                Date: String::from("11/09/2021"),
                //Upvote: db_layer:: 23,
                Upvote: 23,
                Downvote: 1,
            };
            match db_layer::insert_publication(&mut conn, &publication) {
                Ok(last_id) => println!("Inserted product with ID {}", last_id),
                Err(e) => println!("Error: {:?}", e),
            }
            match db_layer::find_publication_by_id(&mut conn, 9) {
                Ok(res) => match res {
                    Some(p) => println!("Found product {}", p.Name),
                    None => println!("Publication not found"),
                },
                Err(e) => println!("Error: {:?}", e),
            }
            let _ = db_layer::get_all_publications(&mut conn).map(|list| {
                for p in list {
                    println!("Found publication {}", p.Name);
                }
            });
        }
    }
}
