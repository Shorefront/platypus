//! Service Catalog Management API Module

use tmflib::tmf633::{
    service_candidate::ServiceCandidate,
    service_specification::ServiceSpecification,
    service_catalog::ServiceCatalog,
    service_category::ServiceCategory,
};

use crate::QueryOptions;

use crate::common::{error::PlatypusError, persist::Persistence};


#[derive(Clone,Default,Debug)]
pub struct TMF633ServiceCatalogManagement {
    persist : Option<Persistence>,
}

impl TMF633ServiceCatalogManagement {
    pub fn new() -> TMF633ServiceCatalogManagement {
        TMF633ServiceCatalogManagement {
            persist: None
        }
    }
    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn get_candidates(&self, query_opts : QueryOptions) -> Result<Vec<ServiceCandidate>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn update_candidate(&self, id : String, patch : ServiceCandidate) -> Result<Vec<ServiceCandidate>,PlatypusError> {
        self.persist.as_ref().unwrap().patch_tmf_item(id, patch).await
    }

    pub async fn get_catalogs(&self, query_opts : QueryOptions) -> Result<Vec<ServiceCatalog>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_categories(&self, query_opts : QueryOptions) -> Result<Vec<ServiceCategory>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_specifications(&self, query_opts : QueryOptions) -> Result<Vec<ServiceSpecification>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn update_specification(&self, id : String, patch : ServiceSpecification) -> Result<Vec<ServiceSpecification>,PlatypusError> {
        self.persist.as_ref().unwrap().patch_tmf_item(id, patch).await
    }
}