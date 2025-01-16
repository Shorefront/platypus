//! Geographic Site Management Module
//! 
//! 

use crate::common::{error::PlatypusError, persist::Persistence};

// Optional modules
#[cfg(feature = "v4")]
use tmflib::tmf674::geographic_site_v4::GeographicSite;
#[cfg(feature = "v5")]
use tmflib::tmf674::geographic_site_v5::GeographicSite;

use crate::QueryOptions;

#[derive(Clone, Debug)]
pub struct TMF674GeographicSiteManagement {
    persist : Option<Persistence>,
}

impl TMF674GeographicSiteManagement {
    pub fn new(persist : Option<Persistence>) -> TMF674GeographicSiteManagement {
        TMF674GeographicSiteManagement {
            persist,
        }
    }

    pub fn persist(&mut self, persist : Persistence) {
        self.persist = Some(persist);
    }

    pub async fn add_site(&self, item : GeographicSite) -> Result<Vec<GeographicSite>,PlatypusError> {
        self.persist.as_ref().unwrap().create_tmf_item(item).await
    }

    pub async fn get_sites(&self, query_opts : QueryOptions) -> Result<Vec<GeographicSite>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_site(&self, id : String, query_opts : QueryOptions) -> Result<Vec<GeographicSite>,PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id,query_opts).await
    }

    pub async fn update_site(&self, id : String, patch : GeographicSite) -> Result<Vec<GeographicSite>,PlatypusError> {
        self.persist.as_ref().unwrap().patch_tmf_item(id, patch).await
    }

    pub async fn delete_site(&self, id : String) -> Result<GeographicSite,PlatypusError> {
        self.persist.as_ref().unwrap().delete_tmf_item(id).await
    }
}