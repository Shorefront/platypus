//! TMF Modules
//! 

use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

use tmflib::HasId;
use crate::common::error::PlatypusError;
use actix_web::HttpResponse;

pub mod tmf620;
#[cfg(feature = "tmf622_v4")]
pub mod tmf622;
pub mod tmf629;
pub mod tmf632;
pub mod tmf648;
#[cfg(feature = "tmf674_v4")]
pub mod tmf674;

pub fn render_list_output<T : Serialize>(output : Result<Vec<T>,PlatypusError>) -> HttpResponse {
    match output {
        Ok(o) => HttpResponse::Ok()
            .append_header(("X-Total-Count",o.len()))
            .json(o),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}

pub fn render_get_output<T : Serialize>(output : Result<Vec<T>,PlatypusError>) -> HttpResponse {
    match output {
        Ok(o) => HttpResponse::Ok()
            .append_header(("X-Total-Count",o.len()))
            .json(o),
        Err(e) => HttpResponse::NotFound().json(e),
    }
}

pub fn render_post_output<T : Serialize + HasId>(output : Result<Vec<T>,PlatypusError>) -> HttpResponse {
    match output {
        Ok(v) => {
            let item = v.first().unwrap();
            HttpResponse::Created()
            .append_header(("Location",item.get_href()))
            .json(item)
        },
        Err(e) => HttpResponse::BadRequest().json(e),   
    }
}

/// Generic TMF struct for DB
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TMF<T : HasId> {
    id : Option<Thing>,
    pub item : T,
}

/// Geneate a TMF payload for storing in the database
pub fn tmf_payload<'a, T : HasId + Serialize + Clone + Deserialize<'a>>(item : T) -> TMF<T> {
    TMF {
        id : Some(Thing {
            tb : T::get_class(),
            id : item.get_id().into(),
        }),
        item,
    }
}