// Web Service Module
use super::db_layer::*;
use actix_web::*;
use mysql::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PublicationQuery {
    pub Id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseQuery {
    pub Status: bool,
}

//Get all the data
#[get("/getAllR")]
pub async fn get_All(data: web::Data<Pool>) -> HttpResponse {
    match data
        .get_conn()
        .and_then(|mut conn| get_all_publications(&mut conn))
    {
        Ok(result_list) => {
            HttpResponse::Ok().json(result_list)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get data by various parameters
// /publication?Id=23&Name=Henry
#[get("/publication")]
pub async fn get_publication_byParams(
    query: web::Query<PublicationQuery>,
    data: web::Data<Pool>,
) -> HttpResponse {
    match data
        .get_conn()
        .and_then(|mut conn| find_publication_by_id(&mut conn, query.Id))
    {
        Ok(result_list) => HttpResponse::Ok().json(result_list),
        Err(_) => HttpResponse::InternalServerError().finish(),
        // Ok(res) => match res {
        //     Some(result_list) => HttpResponse::Ok().json(result_list),
        //     None => HttpResponse::NotFound().finish(),
        // },
        // Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
/* Note these things about the code above.
web :: Query <T> extractor is used to collect the query parameters from the URL.
web: Dataextractor is used to pass shared application data to a controller function. On
In our case, we will configure a group of MySQL connections as shared data.
On error, we return status code 500 using HttpResponse :: InternalServerError (). */

// Get data by a parameter
//publication/12
#[get("/publication/{id}")]
pub async fn getPublication_byId(path: web::Path<u64>, data: web::Data<Pool>) -> HttpResponse {
    println!("entrando a get");
    let id = *path;
    match data
        .get_conn()
        .and_then(|mut conn| find_publication_by_id(&mut conn, id))
    {
        Ok(res) => match res {
            Some(result) => HttpResponse::Ok().json(result),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
/* Here we use the web :: Path <u64> extractor to get the path parameter from the URL. */

// Post by Json
/* This endpoint will use POST to / publication URL and take a Publication JSON as request body.
It will return the newly added JSON from Publication as a response. */
#[post("/publication")]
pub async fn create_publication(publication_json: web::Json<NewPublication>, data: web::Data<Pool>) -> HttpResponse {
    println!("POST");
    let publication = publication_json.into_inner();
 
    match data
        .get_conn()
        .and_then(|mut conn| insert_publication2(&mut conn, &publication))
    {
        Ok(id) => {
            println!("Publicación guardada correctamente");
            //Return a Product with id set.
            // HttpResponse::Ok().json(Publication {
            //     Id: id,
            //     ..publication
            // })
            HttpResponse::Ok().json(id)
        },
        Err(_) => {
            println!("Error al insertar la publicación");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/saveR")]
pub async fn saveR(publication_json: web::Json<NewData>, data: web::Data<Pool>) -> HttpResponse {
    //println!("Iniciando Post");
    let publication = publication_json.into_inner();
    let mut result = ResponseQuery { Status: true };
    match data
        .get_conn()
        .and_then(|mut conn| insert_publication(&mut conn, &publication))
    {
        Ok(id) => {
            //println!("Publicación guardada correctamente, procediendo a insertar hashtags");
            for i in publication.hashtags {
                //println!("{}", i);
                match data
                    .get_conn()
                    .and_then(|mut conn| insert_hashtag(&mut conn, i, id))
                {
                    Ok(_) => {
                        //println!("Hashtag guardado");
                        result.Status = true;
                    }
                    Err(_) => {
                        //println!("Error al insertar el hashtag");
                        result.Status = false;
                    }
                }
            }
            result.Status = true;
            HttpResponse::Ok().json(result)
        }
        Err(_) => {
            //println!("Error al insertar la publicación");
            result.Status = true;
            HttpResponse::Ok().json(result)
            //HttpResponse::InternalServerError().finish()
        }
    }
}
