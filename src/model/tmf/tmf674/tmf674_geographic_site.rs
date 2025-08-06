//! Geographic Site Management Module
//!
//!

use crate::common::{error::PlatypusError, persist::Persistence};

use tmflib::{common::event::EventPayload, tmf674::geographic_site_v4::GeographicSiteEventType};
// Optional modules
#[cfg(feature = "v4")]
use tmflib::tmf674::geographic_site_v4::GeographicSite;
#[cfg(feature = "v5")]
use tmflib::tmf674::geographic_site_v5::GeographicSite;

use crate::QueryOptions;

#[derive(Clone, Debug)]
pub struct TMF674GeographicSiteManagement {
    persist: Option<Persistence>,
}

impl TMF674GeographicSiteManagement {
    pub fn new(persist: Option<Persistence>) -> TMF674GeographicSiteManagement {
        TMF674GeographicSiteManagement { persist }
    }

    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn add_site(
        &self,
        item: GeographicSite,
    ) -> Result<Vec<GeographicSite>, PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .create_tmf_item(item.clone())
            .await;
        #[cfg(feature = "events")]
        {
            let event = item.to_event(GeographicSiteEventType::GeographicSiteCreateEvent);
            let _ = self
                .persist
                .as_ref()
                .unwrap()
                .store_tmf_event(event)
                .await?;
        }
        result
    }

    pub async fn get_sites(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<GeographicSite>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_site(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<GeographicSite>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    pub async fn update_site(
        &self,
        id: String,
        patch: GeographicSite,
    ) -> Result<Vec<GeographicSite>, PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .patch_tmf_item(id, patch.clone())
            .await;
        #[cfg(feature = "events")]
        {
            let event = match patch.status.is_some() {
                true => patch.to_event(GeographicSiteEventType::GeographicSiteStatusChangeEvent),
                false => {
                    patch.to_event(GeographicSiteEventType::GeographicSiteAttributeValueChangeEvent)
                }
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

    pub async fn delete_site(&self, id: String) -> Result<GeographicSite, PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<GeographicSite>(id)
            .await;
        #[cfg(feature = "events")]
        {
            if let Ok(d) = result.clone() {
                let event = d.to_event(GeographicSiteEventType::GeographicSiteDeleteEvent);
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
