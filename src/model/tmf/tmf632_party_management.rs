//! Party Management Module

use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use tmflib::{tmf632::individual::Individual, HasId};
use crate::common::error::PlatypusError;

use super::{tmf_payload,TMF};

pub struct TMF632PartyManagement {
    db : Surreal<Db>,
}

impl TMF632PartyManagement {
    pub fn new(db : Surreal<Db>) -> TMF632PartyManagement {
        TMF632PartyManagement {
            db,
        }
    }
    pub async fn add_individual(&self, individual : Individual) -> Result<TMF<Individual>,PlatypusError> {
        let payload = tmf_payload(individual);
        let insert_records : Vec<TMF<Individual>> = self.db.create(Individual::get_class()).content(payload).await?;
        let record = insert_records.first().unwrap();
        Ok(record.clone())
    }
}