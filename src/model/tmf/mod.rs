//! TMF Modules
//! 

use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use tmflib::HasId;

use crate::common::error::PlatypusError;

pub mod tmf620_catalog_management;
pub mod tmf622_product_order_manaagement;
pub mod tmf632_party_management;

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

/// Generate function to store into a db.
pub async fn store_tmf_item<'a, T : HasId + Serialize + Clone + DeserializeOwned>(db : Surreal<Db>,item : T) -> Result<Vec<T>,PlatypusError> {
    let class = T::get_class();
    let payload = tmf_payload(item);
    let insert_records : Vec<TMF<T>> = db.create(class).content(payload).await?;
    let output = insert_records.into_iter().map(|tmf| {
        tmf.item
    }).collect();
    Ok(output)
}

pub async fn get_tmf_items<T : HasId + Serialize + Clone + DeserializeOwned>(db : Surreal<Db>) -> Result<Vec<T>,PlatypusError> {
    let insert_records : Vec<TMF<T>> = db.select(T::get_class()).await?;
    let output = insert_records.into_iter().map(|tmf| {
        tmf.item.clone()
    }).collect();
    Ok(output)
}

pub async fn get_tmf_item<T : HasId + Serialize + Clone + DeserializeOwned>(db : Surreal<Db>,id : String) -> Result<Vec<T>,PlatypusError> {
    let query = format!("SELECT * FROM {}:{}",T::get_class(),id);
    let mut output = db.query(query).await?;
    let result : Vec<TMF<T>> = output.take(0)?;
    let offer = result.iter().map(|tmf| {
        tmf.clone().item
    }).collect();
    Ok(offer)
}