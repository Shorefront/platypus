//! TMF639 Resource Inventory Management
//! 

use crate::common::{error::PlatypusError, persist::Persistence};
use crate::QueryOptions;
use tmflib::tmf639::resource::Resource;
pub struct TMF639ResourceInventoryManagement {
    persist: Option<Persistence>,
}

impl TMF639ResourceInventoryManagement {
    pub fn new(persist: Option<Persistence>) -> TMF639ResourceInventoryManagement {
        TMF639ResourceInventoryManagement { persist }
    }

    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn get_resources(&self, query_ops: QueryOptions) -> Result<Vec<Resource>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_ops).await
    }

    pub async fn get_resource(&self, id: String, query_ops: QueryOptions) -> Result<Vec<Resource>, PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id, query_ops).await
    }

    pub async fn add_resource(&mut self, item: Resource) -> Result<Vec<Resource>, PlatypusError> {
        let result = self.persist.as_mut().unwrap().create_tmf_item(item.clone()).await;
        #[cfg(feature = "events")]
        {
            // let event = item.to_event();
            // let _ = self.persist.as_ref().unwrap().store_tmf_event(event).await?;
        }
        result
    }

    pub async fn update_resource(&self, id: String, patch: Resource) -> Result<Vec<Resource>, PlatypusError> {
        let result = self.persist.as_ref().unwrap().patch_tmf_item(id, patch.clone()).await;
        #[cfg(feature = "events")]
        {
            // let event = patch.to_event();
            // let _ = self.persist.as_ref().unwrap().store_tmf_event(event).await?;
        }
        result
    }

    pub async fn delete_resource(&self, id: String) -> Result<Resource, PlatypusError> {
        let result = self.persist.as_ref().unwrap().delete_tmf_item::<Resource>(id).await;
        #[cfg(feature = "events")]
        {
            // let event = Service::new(id).to_event();
            // let _ = self.persist.as_ref().unwrap().store_tmf_event(event).await?;
        }
        result
    }
}