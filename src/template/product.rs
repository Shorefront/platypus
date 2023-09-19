//! Product Template
//! 
use tmflib::tmf620::product_offering::ProductOffering;

use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct ProductTemplate {
    offering : ProductOffering,
}