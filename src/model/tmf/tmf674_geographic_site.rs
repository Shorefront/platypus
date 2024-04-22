//! Geographic Site Management Module
//! 
//! 

use crate::common::{error::PlatypusError, persist::Persistence};

// Optional modules
#[cfg(feature = "tmf674_v4")]
use tmflib::tmf674::geographic_site_v4::GeographicSite;

use crate::QueryOptions;

#[derive(Clone, Debug)]
pub struct TMF674GeographicSiteManagement {
    persist : Persistence,
}

impl TMF674GeographicSiteManagement {
    pub fn new(persist : Persistence) -> TMF674GeographicSiteManagement {
        TMF674GeographicSiteManagement {
            persist,
        }
    }

    pub async fn get_sites(&self, query_opts : QueryOptions) -> Result<Vec<GeographicSite>,PlatypusError> {
        self.persist.get_items(query_opts).await
    }

    pub async fn add_site(&mut self, site : GeographicSite) -> Result<Vec<GeographicSite>,PlatypusError> {
        self.persist.create_tmf_item(site).await
    }
}