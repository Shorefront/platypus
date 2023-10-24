//! TMF Modules
//! 

use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

use tmflib::HasId;

pub mod tmf620_catalog_management;
pub mod tmf622_product_order_manaagement;

/// Generic TMF struct for DB
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TMF<T : HasId> {
    id : Option<Thing>,
    item : T,
}

pub fn tmf_payload<T : HasId, Serialize, Deserialize, Clone>(item : T) -> TMF<T> {
    TMF {
        id : Some(Thing {
            tb : item.get_href(),
            id : item.get_id().into(),
        }),
        item,
    }
}