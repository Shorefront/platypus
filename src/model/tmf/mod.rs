//! TMF Modules
//! 

use surrealdb::RecordId;
use serde::{Deserialize, Serialize};

use tmflib::HasId;
use crate::common::error::PlatypusError;
use actix_web::HttpResponse;
use etag::EntityTag;

#[cfg(feature = "tmf620_v4")]
pub mod tmf620;
#[cfg(feature = "tmf622_v4")]
pub mod tmf622;
#[cfg(feature = "tmf629_v4")]
pub mod tmf629;
#[cfg(feature = "tmf632_v4")]
pub mod tmf632;
#[cfg(feature = "tmf633_v4")]
pub mod tmf633;
#[cfg(feature = "tmf648_v4")]
pub mod tmf648;
#[cfg(feature = "tmf674_v4")]
pub mod tmf674;

pub const CONTENT_LANGUAGE : &str = "en_GB";

pub fn render_list_output<T : Serialize>(output : Result<Vec<T>,PlatypusError>) -> HttpResponse {
    match output {
        Ok(o) => HttpResponse::Ok()
            .append_header(("X-Total-Count",o.len()))
            .append_header(("Content-Language",CONTENT_LANGUAGE))
            .json(o),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub fn render_get_output<T : Serialize>(output : Result<Vec<T>,PlatypusError>) -> HttpResponse {
    match output {
        Ok(o) => {
            // Should only be a single result in Vec<> for GET
            let item = o.first();
            match item {
                Some(o) => {
                    let json = serde_json::to_string(o).unwrap();
                    let etag = EntityTag::from_data(json.as_bytes());
                    HttpResponse::Ok()
                    .append_header(("Content-Language",CONTENT_LANGUAGE))
                    .append_header(("ETag",etag.to_string()))
                    .json(o)
                },
                None => {
                    HttpResponse::NotFound().json(PlatypusError::from("Object not found"))    
                }
            }
        },
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub fn render_post_output<T : Serialize + HasId>(output : Result<Vec<T>,PlatypusError>) -> HttpResponse {
    match output {
        Ok(v) => {
            let item = v.first().unwrap();
            HttpResponse::Created()
            .append_header(("Location",item.get_href()))
            .append_header(("Content-Language",CONTENT_LANGUAGE))
            .json(item)
        },
        Err(e) => HttpResponse::BadRequest().json(e),   
    }
}

/// Generic TMF struct for DB
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TMF<T : HasId> {
    id : RecordId,
    pub item : T,
}

/// Geneate a TMF payload for storing in the database
pub fn tmf_payload<'a, T : HasId + Serialize + Clone + Deserialize<'a>>(item : T) -> TMF<T> {
    TMF {
        id : (T::get_class(),item.get_id()).into(),
        item,
    }
}