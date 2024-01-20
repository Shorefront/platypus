//! TMF Modules
//! 

use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

use tmflib::HasId;

pub mod tmf620;
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