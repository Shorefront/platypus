//! TMF622 Product Order Management
//! 
use serde::{Deserialize, Serialize};

/// TMFLIB

#[derive(Clone,Debug, Default, Deserialize, Serialize)]
pub struct TMF622ProductOrderManagement {
    //orders: Vec<ProductOrder>,
    //items: Vec<ProductOrderItem>,
}

impl TMF622ProductOrderManagement {
    pub fn new() -> TMF622ProductOrderManagement {
        TMF622ProductOrderManagement::default()
    }
}