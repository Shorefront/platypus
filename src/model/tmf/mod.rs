//! TMF Modules
//! 

use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

use tmflib::HasId;

#[cfg(feature = "tmf620_v4")]
pub mod tmf620_catalog_management;
#[cfg(feature = "tmf622_v4")]
pub mod tmf622_product_order_management;
#[cfg(feature = "tmf622_v5")]
pub mod tmf622_product_order_management_v5;
#[cfg(feature = "tmf632_v4")]
pub mod tmf632_party_management;
#[cfg(feature = "tmf674_v4")]
pub mod tmf674_geographic_site;

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