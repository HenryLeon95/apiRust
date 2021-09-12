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

//Insertar una publicación del Json
/* Una función que inserta un elemento debe devolver:
El ID generado si la inserción es exitosa. O,
Un error si algo sale mal. La API de MySQL funciona con el mysql::error::Errortipo de errores.*/
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
/* Tenga en cuenta algunas cosas sobre la propiedad:
La insert_product()función recibe una referencia a Productpara evitar mover la propiedad a la función.
Suministramos referencia a codey product_namea la paramsmacro por la misma razón. Podríamos haber clonado las cadenas. Pero eso es una sobrecarga innecesaria. */

//Consulta o Reportes
/* Una función que realiza una consulta puede devolver:
Un vector lleno de elementos.
O un error */
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

//Consulta Por ID
/* Cuando consultamos un elemento por clave primaria, podemos tener tres posibles resultados:
Se encontró el artículo
No se encuentra el artículo
Algún tipo de error
Esto se puede modelar mediante un tipo de retorno std::result::Result<Option<T>, mysql::error::Error>. */
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

// Obtener todos los datos de una tabla
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
