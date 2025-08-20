//! Customer Management Module

use crate::common::{error::PlatypusError, persist::Persistence};

// TMFLIB
use tmflib::{
    common::event::EventPayload,
    tmf629::customer::{Customer, CustomerEventType},
};

use crate::QueryOptions;

pub struct TMF629CustomerManagement {
    persist: Option<Persistence>,
}

impl TMF629CustomerManagement {
    pub fn new(persist: Option<Persistence>) -> TMF629CustomerManagement {
        TMF629CustomerManagement { persist }
    }

    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn get_customers(
        &self,
        query_ops: QueryOptions,
    ) -> Result<Vec<Customer>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_ops).await
    }

    pub async fn get_customer(
        &self,
        id: String,
        query_ops: QueryOptions,
    ) -> Result<Vec<Customer>, PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id, query_ops).await
    }

    pub async fn add_customer(&mut self, item: Customer) -> Result<Vec<Customer>, PlatypusError> {
        let result = self
            .persist
            .as_mut()
            .unwrap()
            .create_tmf_item(item.clone())
            .await;
        #[cfg(feature = "events")]
        {
            let event = item.to_event(CustomerEventType::CustomerCreateEvent);
            let _ = self
                .persist
                .as_ref()
                .unwrap()
                .store_tmf_event(event)
                .await?;
        }
        result
    }

    pub async fn update_customer(
        &self,
        id: String,
        patch: Customer,
    ) -> Result<Vec<Customer>, PlatypusError> {
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
                true => patch.to_event(CustomerEventType::CustomerStateChangeEvent),
                false => patch.to_event(CustomerEventType::CustomerAttributeValueChangeEvent),
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

    pub async fn delete_customer(&self, id: String) -> Result<Customer, PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<Customer>(id)
            .await;
        #[cfg(feature = "events")]
        {
            // Only generate event if successful
            if let Ok(d) = result.clone() {
                let event = d.to_event(CustomerEventType::CustomerDeleteEvent);
                let _ = self
                    .persist
                    .as_ref()
                    .unwrap()
                    .store_tmf_event(event)
                    .await?;
            }
        }
        result
    }
}
