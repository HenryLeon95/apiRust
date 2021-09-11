#![allow(non_snake_case)]
use mysql::prelude::*;
use mysql::*; //For date and time

fn main() {
    dotenv::dotenv().ok();
    //let port = std::env::var("PORT").unwrap_or("4001".to_string());
    //let address = format!("127.0.0.1:{}", port);
    let user_db = std::env::var("USER_DB")
        .unwrap_or("mysql".to_string())
        .to_string();
    let pass = std::env::var("PASS")
        .unwrap_or("1234".to_string())
        .to_string();
    let host = std::env::var("HOST")
        .unwrap_or("localhost".to_string())
        .to_string();
    let port_db = std::env::var("PORT_DB")
        .unwrap_or("3306".to_string())
        .to_string()
        .parse::<u16>()
        .unwrap();
    let database = std::env::var("DATABASE")
        .unwrap_or("mysql".to_string())
        .to_string();

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user_db, pass, host, port_db, database
    );
    let opts = Opts::from_url(&url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    //Una consulta simple
    conn.query_iter("select id, nombre, comentario from publicaciones")
        .unwrap()
        .for_each(|row| {
            let r: (i32, String, String) = from_row(row.unwrap());
            println!("{}, {}, {}", r.0, r.1, r.2);
        });

    //Consulta guardando en un vector
    let res: Vec<(i32, String, String)> = conn
        .query("select id, nombre, comentario from publicaciones")
        .unwrap();
    for r in res {
        println!("{}, {}, {}", r.0, r.1, r.2);
    }
}
