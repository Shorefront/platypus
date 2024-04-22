//! Party Management Module

use tmflib::common::event::EventPayload;
use tmflib::tmf632::individual::IndividualEventType;
use tmflib::tmf632::organization::OrganizationEventType;
use tmflib::tmf632::{
        individual::Individual,
        organization::Organization,
    }
;
use crate::common::{error::PlatypusError, persist::Persistence};
use crate::QueryOptions;

use log::{debug,error};

#[derive(Clone, Debug)]
pub struct TMF632PartyManagement {
    persist : Persistence,
}

impl TMF632PartyManagement {
    pub fn new(persist : Persistence) -> TMF632PartyManagement {
        TMF632PartyManagement {
            persist,
        }
    }
    fn party_exists(&self, party_id : String) -> Result<String,PlatypusError> {
        // Confirm if the party exists in the DB
        Ok(party_id)
    }
    fn validate_individual(&self, individual : &Individual) -> Result<bool,PlatypusError> {
        let mut err_count = 0;
        if individual.related_party.is_some() {
            // There is some here, lets iterate and validate each related party
            individual.related_party.as_ref().unwrap().into_iter().for_each(|rp| {
                if self.party_exists(rp.id.clone()).is_err() {
                    err_count += 1;
                }
            });
        }
        if err_count > 0 {
            return Err(PlatypusError::from("TMF632: Invalid related party for individual"));
        }
        Ok(true) 
    }

    pub async fn add_organization(&mut self, organization : Organization) -> Result<Vec<Organization>,PlatypusError> {
        let out = self.persist.create_tmf_item(organization.clone()).await;

        // Generate an event
        let event = organization.to_event(OrganizationEventType::OrganizationCreateEvent);
        let _event_result = self.persist.send_tmf_event(event);

        out
    }

    pub async fn add_individual(&mut self, individual : Individual) -> Result<Vec<Individual>,PlatypusError> {
        match self.validate_individual(&individual) {
            Ok(_) => debug!("Individual validated"),
            Err(e) => {
                error!("Individual failed validation: {}",e);
                return Err(e);
            }
        };

        let out = self.persist.create_tmf_item(individual.clone()).await;

        // Now that we've added the record to the persistence layer we can generate a create event.
        let event = individual.to_event(IndividualEventType::IndividualCreateEvent);
        // Send event if anyone is listening for it.
        let _event_result = self.persist.send_tmf_event(event).await;
        out
    }

    pub async fn get_individuals(&self,query_opts : QueryOptions) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.get_items(query_opts).await
    }

    pub async fn get_individual(&self, id : String, query_opts : QueryOptions) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.get_item(id,query_opts).await
    }
}