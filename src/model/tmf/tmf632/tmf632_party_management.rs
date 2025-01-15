//! Party Management Module

use tmflib::tmf632::organization_v4::Organization;
#[cfg(all(feature = "tmf632",feature="v4"))]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(all(feature = "tmf632",feature="v5"))]
use tmflib::tmf632::individual_v5::Individual;
use crate::common::{error::PlatypusError, persist::Persistence};
use crate::QueryOptions;

#[derive(Clone, Debug)]
pub struct TMF632PartyManagement {
    persist : Option<Persistence>,
}

impl TMF632PartyManagement {
    pub fn new(persist : Option<Persistence>) -> TMF632PartyManagement {
        TMF632PartyManagement {
            persist,
        }
    }

    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn add_individual(&mut self, individual : Individual) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.as_mut().unwrap().create_tmf_item(individual).await
    }

    pub async fn get_individuals(&mut self,query_opts : QueryOptions) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.as_mut().unwrap().get_items(query_opts).await
    }

    pub async fn get_individual(&mut self, id : String, query_opts : QueryOptions) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.as_mut().unwrap().get_item(id,query_opts).await
    }

    pub async fn update_individual(&self, id : String, patch : Individual) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.as_ref().unwrap().patch_tmf_item(id, patch).await
    }

    pub async fn delete_individual(&mut self, id : String) -> Result<bool,PlatypusError> {
        self.persist.as_mut().unwrap().delete_tmf_item::<Individual>(id).await
    }

    pub async fn add_organization(&mut self, organization : Organization) -> Result<Vec<Organization>,PlatypusError> {
        self.persist.as_mut().unwrap().create_tmf_item(organization).await
    }

    pub async fn get_organizations(&mut self, query_opts : QueryOptions) -> Result<Vec<Organization>,PlatypusError> {
        self.persist.as_mut().unwrap().get_items(query_opts).await
    }

    pub async fn get_organization(&mut self, id : String, query_opts : QueryOptions) -> Result<Vec<Organization>,PlatypusError> {
        self.persist.as_mut().unwrap().get_item(id,query_opts).await
    }

    pub async fn update_organization(&self, id : String, patch : Organization) -> Result<Vec<Organization>,PlatypusError> {
        self.persist.as_ref().unwrap().patch_tmf_item(id, patch).await
    }

    pub async fn delete_organization(&mut self, id : String) -> Result<bool,PlatypusError> {
        self.persist.as_mut().unwrap().delete_tmf_item::<Organization>(id).await
    }
}