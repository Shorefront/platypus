//! TMF638 Service Inventory Management

use crate::common::{error::PlatypusError, persist::Persistence};
use crate::QueryOptions;
use tmflib::tmf638::service::Service;

pub struct TMF638ServiceInventoryManagement {
    persist: Option<Persistence>,
}

impl TMF638ServiceInventoryManagement {
    pub fn new(persist: Option<Persistence>) -> TMF638ServiceInventoryManagement {
        TMF638ServiceInventoryManagement { persist }
    }

    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn get_services(&self, query_ops: QueryOptions) -> Result<Vec<Service>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_ops).await
    }

    pub async fn get_service(&self, id: String, query_ops: QueryOptions) -> Result<Vec<Service>, PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id, query_ops).await
    }

    pub async fn add_service(&mut self, item: Service) -> Result<Vec<Service>, PlatypusError> {
        let result = self.persist.as_mut().unwrap().create_tmf_item(item.clone()).await;
        #[cfg(feature = "events")]
        {
            // let event = item.to_event();
            // let _ = self.persist.as_ref().unwrap().store_tmf_event(event).await?;
        }
        result
    }

    pub async fn update_service(&self, id: String, patch: Service) -> Result<Vec<Service>, PlatypusError> {
        let result = self.persist.as_ref().unwrap().patch_tmf_item(id, patch.clone()).await;
        #[cfg(feature = "events")]
        {
            // let event = patch.to_event();
            // let _ = self.persist.as_ref().unwrap().store_tmf_event(event).await?;
        }
        result
    }

    pub async fn delete_service(&self, id: String) -> Result<Service, PlatypusError> {
        let result = self.persist.as_ref().unwrap().delete_tmf_item::<Service>(id).await;
        #[cfg(feature = "events")]
        {
            // let event = Service::new(id).to_event();
            // let _ = self.persist.as_ref().unwrap().store_tmf_event(event).await?;
        }
        result
    }
}