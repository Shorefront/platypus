//! TMF637 Product Inventory Management API

use crate::common::{error::PlatypusError, persist::Persistence};
use crate::QueryOptions;
use tmflib::tmf637::v4::product::Product;

pub struct TMF637ProductInventoryManagement {
    persist: Option<Persistence>,
}

impl TMF637ProductInventoryManagement {
    pub fn new(persist: Option<Persistence>) -> TMF637ProductInventoryManagement {
        TMF637ProductInventoryManagement { persist }
    }

    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn get_products(
        &self,
        query_ops: QueryOptions,
    ) -> Result<Vec<Product>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_ops).await
    }

    pub async fn get_product(
        &self,
        id: String,
        query_ops: QueryOptions,
    ) -> Result<Vec<Product>, PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id, query_ops).await
    }

    pub async fn add_product(&mut self, item: Product) -> Result<Vec<Product>, PlatypusError> {
        let result = self
            .persist
            .as_mut()
            .unwrap()
            .create_tmf_item(item.clone())
            .await;
        #[cfg(feature = "events")]
        {
            let event = item.to_event(ProductEventType::ProductCreateEvent);
            let _ = self
                .persist
                .as_ref()
                .unwrap()
                .store_tmf_event(event)
                .await?;
        }
        result
    }

    pub async fn update_product(
        &self,
        id: String,
        patch: Product,
    ) -> Result<Vec<Product>, PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .patch_tmf_item(id, patch.clone())
            .await;
        #[cfg(feature = "events")]
        {
            // Need to determine if the state has changed to set the correct event type
            let event = match patch.status.is_some() {
                true => patch.to_event(ProductEventType::ProductStateChangeEvent),
                false => patch.to_event(ProductEventType::ProductAttributeValueChangeEvent),
            };
            let _ = self
                .persist
                .as_ref()
                .unwrap()
                .store_tmf_event(event)
                .await?;
        }
        result
    }

    pub async fn delete_product(&self, id: String) -> Result<Product, PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<Product>(id)
            .await;
        #[cfg(feature = "events")]
        {
            let event = Product::new(id).to_event(ProductEventType::ProductDeleteEvent);
            let _ = self
                .persist
                .as_ref()
                .unwrap()
                .store_tmf_event(event)
                .await?;
        }
        result
    }
}
