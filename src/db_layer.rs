use mysql::prelude::*;
use mysql::*;

pub struct Publication {
    pub Id: u64,
    pub Name: String,
    pub Comment: String,
    pub Date: String,
    pub Upvote: u64,
    pub Downvote: u64,
}

// Insert a Json post
/* A function that inserts an element should return:
The ID generated if the insert is successful. OR,
An error if something goes wrong. The MySQL API works with the mysql :: error :: Error type. */
pub fn insert_publication(
    conn: &mut PooledConn,
    publication: &Publication,
) -> std::result::Result<u64, mysql::error::Error> {
    conn.exec_drop(
        "insert into publicaciones (nombre, comentario, fecha, upvotes, downvotes) values (:nombre, :comentario, :fecha, :upvotes, :downvotes)",
        params! {
            "nombre" => &publication.Name,
            "comentario" => &publication.Comment,
            "fecha" => &publication.Date,
            "upvotes" => &publication.Upvote,
            "downvotes" => &publication.Downvote,
        },
    )
    .and_then(|_| Ok(conn.last_insert_id()))
}
// Insert to Json post
/* A function that inserts an element should return:
The ID generated if the insert is successful. OR,
An error if something goes wrong. The MySQL API works with the mysql :: error :: Error type. */

// Query or Reports
/* A function that performs a query can return:
A vector full of elements.
Or an error */
// pub fn find_product_in_(
//     conn: &mut PooledConn,
//     price_from: f32,
//     price_to: f32) -> std::result::Result<Vec<Product>, mysql::error::Error> {
//     conn.exec_map(
//         "select product_id, product_code, price, name, last_update from PRODUCT where price>=:price_from and price <=:price_to",
//         params! {
//             "price_from" => price_from,
//             "price_to" => price_to,
//         },
//         |(product_id, product_code, price, name, last_update)| Product {
//             id: product_id,
//             code: product_code,
//             price: price,
//             product_name: name,
//             last_changed_on: last_update
//         }
//     )
// }

// Query By ID
/* When we query an element by primary key, we can have three possible results:
Item Found
Item not found
Some kind of mistake
This can be modeled by a return type std :: result :: Result <Option <T>, mysql :: error :: Error>. */
pub fn find_publication_by_id(
    conn: &mut PooledConn,
    id: u64,
) -> std::result::Result<Option<Publication>, mysql::error::Error> {
    let row = conn.exec_first(
        "select id, nombre, comentario, fecha, upvotes, downvotes from publicaciones WHERE id =:id",
        params! {
            "id" => id
        },
    )?;
    Ok(
        row.map(|(Id, Name, Comment, Date, Upvote, Downvote)| Publication {
            Id: Id,
            Name: Name,
            Comment: Comment,
            Date: Date,
            Upvote: Upvote,
            Downvote: Downvote,
        }),
    )
}

// Get all the data from a table
pub fn get_all_publications(
    conn: &mut PooledConn,
) -> std::result::Result<Vec<Publication>, mysql::error::Error> {
    conn.query_map(
        "select id, nombre, comentario, fecha, upvotes, downvotes from publicaciones",
        |(Id, Name, Comment, Date, Upvote, Downvote)| Publication {
            Id: Id,
            Name: Name,
            Comment: Comment,
            Date: Date,
            Upvote: Upvote,
            Downvote: Downvote,
        },
    )
}
