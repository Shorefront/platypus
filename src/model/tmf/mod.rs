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

/// Generic TMF struct for DB
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TMF<T : HasId> {
    id : Option<Thing>,
    pub item : T,
}