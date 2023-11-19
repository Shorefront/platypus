//! TMF622 Product Order Management
//! 
use serde::{Deserialize, Serialize};

/// TMFLIB
use tmflib::tmf622::product_order::ProductOrder;
use tmflib::tmf622::product_order_item::ProductOrderItem;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TMF622ProductOrderManagement {
    orders: Vec<ProductOrder>,
    items: Vec<ProductOrderItem>,
}