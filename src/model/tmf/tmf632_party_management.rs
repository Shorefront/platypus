//! Party Management Module

use tmflib::{tmf632::individual::Individual, HasId};
use crate::common::{error::PlatypusError, persist::Persistence};

use super::{tmf_payload,TMF};

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
    pub async fn add_individual(&self, individual : Individual) -> Result<Individual,PlatypusError> {
        match self.validate_individual(&individual) {
            Ok(_) => debug!("Individual validated"),
            Err(e) => {
                error!("Individual failed validation: {}",e);
                return Err(e);
            }
        };
        let payload = tmf_payload(individual);
        let insert_records : Vec<TMF<Individual>> = self.persist.db.create(Individual::get_class()).content(payload).await?;
        let record = insert_records.first().unwrap();
        Ok(record.item.clone())
    }

    pub async fn get_individuals(&self) -> Result<Vec<Individual>,PlatypusError> {
        let get_records : Vec<TMF<Individual>> = self.persist.db.select(Individual::get_class()).await?;
        let mut output : Vec<Individual> = vec![];
        
        // Need to generate a vec of sub_categories
        get_records.iter().for_each(|ir| {
            output.push(ir.item.clone());
        });
        Ok(output)
    }
}