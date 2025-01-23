//! Quote Management Module

use crate::common::{error::PlatypusError, persist::Persistence};
use crate::QueryOptions;
use tmflib::common::event::EventPayload;
use tmflib::tmf648::quote::{Quote, QuoteEventType};
use log::debug;

pub struct TMF648QuoteManagement {
    persist : Option<Persistence>,
}

impl TMF648QuoteManagement {
    pub fn new(persist : Option<Persistence>) -> TMF648QuoteManagement {
        TMF648QuoteManagement {
            persist,
        }
    }

    pub fn persist(&mut self, persist : Persistence) {
        self.persist = Some(persist);
    }

    pub async fn get_quotes(&self, query_opts : QueryOptions) -> Result<Vec<Quote>,PlatypusError> {
        debug!("Getting quotes");
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_quote(&self, id : String, query_opts : QueryOptions) -> Result<Vec<Quote>,PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id,query_opts).await
    }

    pub async fn add_quote(&mut self, item : Quote) -> Result<Vec<Quote>,PlatypusError> {
        let result = self.persist.as_mut().unwrap().create_tmf_item(item.clone()).await;
        #[cfg(feature = "events")]
        {
            let event = item.to_event(QuoteEventType::QuoteCreateEvent);
            let _ = self.persist.as_ref().unwrap().store_tmf_event(event);
        }
        result
    }

    pub async fn update_quote(&self, id : String, patch : Quote) -> Result<Vec<Quote>,PlatypusError> {
        self.persist.as_ref().unwrap().patch_tmf_item(id, patch).await
    }

    pub async fn delete_quote(&self, id : String) -> Result<Quote,PlatypusError> {
        let result = self.persist.as_ref().unwrap().delete_tmf_item::<Quote>(id).await;
        #[cfg(feature = "events")]
        {
            if let Ok(d) = result.clone() {
                let event = d.to_event(QuoteEventType::QuoteDeleteEvent);
                let _ = self.persist.as_ref().unwrap().store_tmf_event(event);
            }
        }
        result
    }
}