#![allow(non_snake_case)]

extern crate dotenv;
//use actix_web -> Import a series of elementor
//use std::sync::atomic::{AtomicU16, Ordering};
//use std::sync::Arc;
use actix_web::{web, App, HttpServer, Responder};

/*
let tweet = models::Tweet {
    name: String::from(("prueba"),
    msg: String::from("mensaje"),
};*/


async fn get() -> impl Responder {
    format!("API RUST")
}


async fn getAll() -> impl Responder {
    format!("Obteniendo todos los tweets")
}




/*
// Asynchronous Function that Returns a String
// Gets the name parameter of the request and if it does not exist, adds the value World
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}*/

// Entry point
// Returns a Result of type IO.
// If everything goes well it returns a unit type. If there is an error, it returns an IO type.
// All the code with the decorator is equal to:
// fn main() -> std::io::Result<()> {
//     actix_web::rt::System::new("main").block_on(
//         async {
//             HttpServer::new(|| {
//                 App::new()
//                     .route("/", web::get().to(greet))
//                     .route("/{name}", web::get().to(greet))
//             })
//             .bind(("127.0.0.1", 5001))?
//             .run()
//             .await
//         }
//     )
// }

// Decorator, procedural macro. Grab all the code below and plug it into a tokyo times.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("4001".to_string());
    let address = format!("127.0.0.1:{}", port);
    println!("Starting our server");

    // Initializing Atomic Integer (It can be used safely in all threads to be executed)
    // let thread_counter = Arc::new(AtomicU16::new(1));

    // Move converts any variable captured by reference or muteable reference to variables captured by value.
    HttpServer::new(move || {
        // The number of times of the Core's that we have is run
        // println!(
        //     "Starting thread {}",
        //     //fetch_add -> ends up mutating the value of the counter, adds 1 and returns it for each core
        //     thread_counter.fetch_add(1, Ordering::SeqCst)
        // );
        // let thread_index = thread_counter.load(Ordering::SeqCst);

        App::new()
            .route("/", web::get().to(get))
            .route("/getAll", web::get().to(getAll))
            .route("/{name}", web::get().to(get))

            /*
            // Test Routes
            .route("/test", web::get().to(greet))
            // In case we want a Route to be shared in the same thread, the variable thread_index is used
            .route("/health", web::get().to(move || {
                HttpResponse::Ok()
                    .header("thread-id", thread_index.to_string())
                    .finish()
                }),
            )
            //ASYMC -> Convert any instance within async to a Future
            .route("/str", web::get().to(|| async { "Hello Rust" }))*/
    })
    .bind(&address)?
    //.workers(1) //Indicate the amount of cores we want to use
    .run()
    .await
}
