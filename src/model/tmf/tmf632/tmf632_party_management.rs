//! Party Management Module

use tmflib::common::event::EventPayload;
use tmflib::tmf632::individual_v4::IndividualEventType;
use tmflib::tmf632::organization_v4::{Organization, OrganizationEventType};
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

    pub async fn add_individual(&self, individual : Individual) -> Result<Vec<Individual>,PlatypusError> {
        let result = self.persist.as_ref().unwrap().create_tmf_item(individual.clone()).await;
        #[cfg(feature = "events")]
        {
            let event = individual.to_event(IndividualEventType::IndividualCreateEvent);
            let _ = self.persist.as_ref().unwrap().store_tmf_event(event);
        }
        result
    }

    pub async fn get_individuals(&self,query_opts : QueryOptions) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_individual(&self, id : String, query_opts : QueryOptions) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id,query_opts).await
    }

    pub async fn update_individual(&self, id : String, patch : Individual) -> Result<Vec<Individual>,PlatypusError> {
        let result = self.persist.as_ref().unwrap().patch_tmf_item(id, patch.clone()).await;
        #[cfg(feature = "events")]
        {
            // Determine if the status is being updated to set the correct event type
            // TODO: No status field present to check
            let event = patch.to_event(IndividualEventType::IndividualAttributeValueChangeEvent);
            let _ = self.persist.as_ref().unwrap().store_tmf_event(event);
        }
        result
    }

    pub async fn delete_individual(&mut self, id : String) -> Result<Individual,PlatypusError> {
        let result = self.persist.as_mut().unwrap().delete_tmf_item::<Individual>(id).await;
        #[cfg(feature = "events")]
        {
            if let Ok(d) = result.clone() {
                let event = d.to_event(IndividualEventType::IndividualDeleteEvent);
                let _ = self.persist.as_ref().unwrap().store_tmf_event(event);
            }
        }
        result
    }

    pub async fn add_organization(&self, organization : Organization) -> Result<Vec<Organization>,PlatypusError> {
        let result = self.persist.as_ref().unwrap().create_tmf_item(organization.clone()).await;
        #[cfg(feature = "events")]
        {
            let event = organization.to_event(OrganizationEventType::OrganizationCreateEvent);
            let _ = self.persist.as_ref().unwrap().store_tmf_event(event);
        }
        result
    }

    pub async fn get_organizations(&self, query_opts : QueryOptions) -> Result<Vec<Organization>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_organization(&self, id : String, query_opts : QueryOptions) -> Result<Vec<Organization>,PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id,query_opts).await
    }

    pub async fn update_organization(&self, id : String, patch : Organization) -> Result<Vec<Organization>,PlatypusError> {
        self.persist.as_ref().unwrap().patch_tmf_item(id, patch).await
    }

    pub async fn delete_organization(&self, id : String) -> Result<Organization,PlatypusError> {
        let result = self.persist.as_ref().unwrap().delete_tmf_item::<Organization>(id).await;
        #[cfg(feature = "events")]
        {
            if let Ok(d) = result.clone() {
                let event = d.to_event(OrganizationEventType::OrganizationDeleteEvent);
                let _ = self.persist.as_ref().unwrap().store_tmf_event(event);
            }
        }
        result
    }
}