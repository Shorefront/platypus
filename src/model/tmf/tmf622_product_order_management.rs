//! TMF622 Product Order Management
//! 
use serde::{Deserialize, Serialize};

/// TMFLIB
///
#[cfg(feature = "v4")]
use tmflib::tmf622::product_order_v4::ProductOrder;

#[cfg(feature = "v5")]
use tmflib::tmf622::product_order_v5::ProductOrder;

use tmflib::tmf622::product_order_item::ProductOrderItem;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TMF622ProductOrderManagement {
    orders: Vec<ProductOrder>,
    items: Vec<ProductOrderItem>,
}