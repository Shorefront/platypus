//! Party Management Module

use tmflib::tmf632::organization_v4::Organization;
use tmflib::HasId;
#[cfg(feature = "tmf632_v4")]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(feature = "tmf632_v5")]
use tmflib::tmf632::individual_v5::Individual;
use crate::common::{error::PlatypusError, persist::Persistence};
use crate::QueryOptions;

use crate::model::tmf::{tmf_payload,TMF};

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
    fn party_exists(&self, party_id : String) -> Result<String,PlatypusError> {
        // Confirm if the party exists in the DB
        Ok(party_id)
    }
    fn validate_individual(&self, individual : &Individual) -> Result<Individual,PlatypusError> {
        let mut err_count = 0;
        if individual.related_party.is_some() {
            // There is some here, lets iterate and validate each related party
            individual.related_party.as_ref().unwrap().iter().for_each(|rp| {
                if self.party_exists(rp.id.clone()).is_err() {
                    err_count += 1;
                }
            });
        }
        if err_count > 0 {
            return Err(PlatypusError::from("TMF632: Invalid related party for individual"));
        }
        Ok(individual.clone()) 
    }
    pub async fn add_individual(&mut self, individual : Individual) -> Result<Vec<Individual>,PlatypusError> {
        let individual = self.validate_individual(&individual)?;
        let payload = tmf_payload(individual);
        let insert_records : Vec<TMF<Individual>> = self.persist.as_mut().unwrap().db.create(Individual::get_class()).content(payload).await?;
        let records : Vec<Individual> = insert_records.into_iter().map(|r| r.item).collect();
        Ok(records)
    }

    pub async fn get_individuals(&mut self,query_opts : QueryOptions) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.as_mut().unwrap().get_items(query_opts).await
    }

    pub async fn get_individual(&mut self, id : String, query_opts : QueryOptions) -> Result<Vec<Individual>,PlatypusError> {
        self.persist.as_mut().unwrap().get_item(id,query_opts).await
    }

    pub async fn add_organization(&mut self, organization : Organization) -> Result<Vec<Organization>,PlatypusError> {
        let tmf_payload = tmf_payload(organization);
        let insert_records : Vec<TMF<Organization>> = self.persist.as_mut().unwrap().db.create(Organization::get_class()).content(tmf_payload).await?;
        let tmf_records : Vec<Organization> = insert_records.into_iter().map(|r| r.item).collect();
        Ok(tmf_records)
    }

    pub async fn get_organizations(&mut self, query_opts : QueryOptions) -> Result<Vec<Organization>,PlatypusError> {
        self.persist.as_mut().unwrap().get_items(query_opts).await
    }

    pub async fn get_organization(&mut self, id : String, query_opts : QueryOptions) -> Result<Vec<Organization>,PlatypusError> {
        self.persist.as_mut().unwrap().get_item(id,query_opts).await
    }
}