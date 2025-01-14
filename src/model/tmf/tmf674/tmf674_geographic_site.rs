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

    pub async fn get_sites(&self, query_opts : QueryOptions) -> Result<Vec<GeographicSite>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn add_site(&mut self, item : GeographicSite) -> Result<Vec<GeographicSite>,PlatypusError> {
        self.persist.as_mut().unwrap().create_tmf_item(item).await
    }
}