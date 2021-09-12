#![allow(non_snake_case)]
use mysql::prelude::*;
use mysql::*;

//Structs
struct Publication {
    Id: u64,
    Name: String,
    Comment: String,
    Date: String,
    Upvote: u64,
    Downvote: u64,
    //last_changed_on: NaiveDate, in case you want a date
}

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
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // A Simple Query - Query Transmitted
    // conn.query_iter("select id, nombre, comentario from publicaciones")
    //     .unwrap()
    //     .for_each(|row| {
    //         let r: (i32, String, String) = from_row(row.unwrap());
    //         println!("{}, {}, {}", r.0, r.1, r.2);
    //     });
    /* Here rowes of type mysql_common :: row :: Row. This type carries data in the form of bytes
     very low level. The form_row () function is used to convert the bytes into something more friendly
     as i32y String. The converted data is returned in a tuple where the elements are
     in the same order as the columns selected in the query. */


    //Query saving to vector - Collecting query result
    // let res: Vec<(i32, String, String)> = conn
    //     .query("select id, nombre, comentario from publicaciones")
    //     .unwrap();
    // for r in res {
    //     println!("{}, {}, {}", r.0, r.1, r.2);
    // }
    /* The query () function already converts the low-level bytes to our choice of data types
     so we don't have to do that. We had to explicitly mention the data type of the
     tuple. Otherwise the compiler has no way of knowing. */


    //Query using structs - Convert result to structured data
    /* Now we can map the query result to the Productobjects using query_map (). */
    // let res = conn
    //     .query_map(
    //         "select id, nombre, comentario, fecha, upvotes, downvotes from publicaciones",
    //         |(Id, Name, Comment, Date, Upvote, Downvote)| Publication {
    //             Id: Id,
    //             Name: Name,
    //             Comment: Comment,
    //             Date: Date,
    //             Upvote: Upvote,
    //             Downvote: Downvote,
    //         },
    //     )
    //     .expect("Query failed.");
    // println!("si paso");
    // for r in res {
    //     println!(
    //         "{}, {}, {}, {}, {}, {}",
    //         r.Id, r.Name, r.Comment, r.Date, r.Upvote, r.Downvote
    //     );
    // }
    /* The good thing is that we didn't have to specify the data type of the tuple. The compiler deduced it
     of the data types of the Product fields. */


    // Single element query.
    /* As a result, the query_first () function returns a Resultde Option. We need to unzip it
    twice to get the actual data for the row. */
    // let res = conn
    // .query_first("select id, nombre, comentario, fecha, upvotes, downvotes from publicaciones where id=8")
    // /* If you want to use named parameters, use the exec_first / () function instead of query_first */
    // // .exec_first(
    // //     "select id, nombre, comentario, fecha, upvotes, downvotes from publicaciones where ied=:id",
    // //     params!{
    // //         "id"=> 8
    // //     }, 
    // // )
    // //Unpack Result
    // .map(|row| {
    //     //Unpack Option
    //     row.map(|(Id, Name, Comment, Date, Upvote, Downvote)| Publication {
    //         Id: Id,
    //         Name: Name,
    //         Comment: Comment,
    //         Date: Date,
    //         Upvote: Upvote,
    //         Downvote: Downvote,
    //     })
    // });
    // match res.unwrap() {
    //     Some(publi) => println!(
    //         "{}, {}, {}, {}, {}, {}", // {:?} is in case the data is type date
    //         publi.Id, publi.Name, publi.Comment, publi.Date, publi.Upvote, publi.Downvote
    //     ),
    //     None => println!("Sorry no publication found."),
    // }


    // Inserting data
    // This is in case they need to use the date.
    // fn today() -> NaiveDate {
    //     let l = Local::today();
    //     NaiveDate::from_ymd(l.year(), l.month(), l.day())
    // }
    // let stmt = conn.prep("insert into publicaciones (nombre, comentario, fecha, upvotes, downvotes) values (:nombre, :comentario, :fecha, :upvotes, :downvotes)")
    //     .unwrap();
    // conn.exec_drop(
    //     &stmt,
    //     params! {
    //         "nombre" => "Rust 1",
    //         "comentario" => "Comentario simple de Rust 1",
    //         "fecha" => "11/09/2021",
    //         //Pos i se desea usar la fecha "upvotes" => today(),
    //         "upvotes" => 15,
    //         "downvotes" => 42,
    //     },
    // )
    // .unwrap();
    // conn.exec_drop(
    //     &stmt,
    //     params! {
    //         "nombre" => "Rust 1",
    //         "comentario" => "Comentario simple de Rust 2",
    //         "fecha" => "11/09/2021",
    //         //Pos i se desea usar la fecha "upvotes" => today(),
    //         "upvotes" => 5,
    //         "downvotes" => 420,
    //     },
    // )
    // .unwrap();
    /* The paramsmacro makes it easy to supply named parameter values. Here drop exec_drop ()
     means that no result data is returned. Which is fine for inserting, updating, and
     remove the SQL type.
     If you do a lot of inserts, it will be faster to compile the SQL as a prepared statement. */


    // Get generated primary key
    // let stmt = conn.prep("insert into publicaciones (nombre, comentario, fecha, upvotes, downvotes) values (:nombre, :comentario, :fecha, :upvotes, :downvotes)")
    //     .unwrap();
    // conn.exec_drop(
    //     &stmt,
    //     params! {
    //         "nombre" => "Rust 1",
    //         "comentario" => "Comentario simple de Rust 3",
    //         "fecha" => "11/09/2021",
    //         "upvotes" => 70,
    //         "downvotes" => 2,
    //     },
    // ).unwrap();
    // println!("Last generated key: {}", conn.last_insert_id());
    /* In our case, the primary key is generated by MySQL. We can get it back by calling conn.last_insert_id (). */


    // Update and Delete
    /* They are similar to insert, example of update and delete respectively */
    // let stmt = conn
    //     .prep("update publicaciones set nombre=:nombre, comentario=:comentario where id=:id")
    //     .unwrap();
    // conn.exec_drop(
    //     &stmt,
    //     params! {
    //         "id" => 32,
    //         "nombre" => "Rust 1",
    //         "comentario" => "Comentario simple de Rust 3 modificado",
    //         //"last_update" => NaiveDate::from_ymd(2020, 10, 12), Por si se desea la fecha
    //     },
    // )
    // .unwrap();

    // let stmt = conn
    //     .prep("delete from publicaciones where id=:id")
    //     .unwrap();
    // conn.exec_drop(
    //     &stmt,
    //     params! {
    //         "id" => 21,
    //     },
    // )
    // .unwrap();
}
