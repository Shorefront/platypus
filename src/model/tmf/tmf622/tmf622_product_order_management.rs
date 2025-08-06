//! TMF622 Product Order Management
//!
use tmflib::tmf622::product_order_v4::ProductOrder;

use crate::{
    common::{error::PlatypusError, persist::Persistence},
    QueryOptions,
};

/// TMFLIB

#[derive(Clone, Debug, Default)]
pub struct TMF622ProductOrderManagement {
    persist: Option<Persistence>,
}

impl TMF622ProductOrderManagement {
    pub fn new(persist: Option<Persistence>) -> TMF622ProductOrderManagement {
        TMF622ProductOrderManagement { persist }
    }

    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn add_order(&self, item: ProductOrder) -> Result<Vec<ProductOrder>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .create_tmf_item_lastupdate(item)
            .await
    }

    pub async fn get_orders(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<ProductOrder>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_order(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<ProductOrder>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    pub async fn update_order(
        &self,
        id: String,
        patch: ProductOrder,
    ) -> Result<Vec<ProductOrder>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item(id, patch)
            .await
    }

    pub async fn delete_order(&self, id: String) -> Result<ProductOrder, PlatypusError> {
        self.persist.as_ref().unwrap().delete_tmf_item(id).await
    }
}
