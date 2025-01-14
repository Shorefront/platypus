//! Customer Management Module

use crate::common::{error::PlatypusError, persist::Persistence};

// TMFLIB
use tmflib::tmf629::customer::Customer;

use crate::QueryOptions;

pub struct TMF629CustomerManagement {
    persist : Option<Persistence>,
}

impl TMF629CustomerManagement {
    pub fn new(persist : Option<Persistence>) -> TMF629CustomerManagement {
        TMF629CustomerManagement { persist }
    }

    pub fn persist(&mut self, persist : Persistence) {
        self.persist = Some(persist);
    }

    pub async fn get_customers(&self, query_ops : QueryOptions) -> Result<Vec<Customer>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_ops).await
    }

    pub async fn get_customer(&self, id : String, query_ops : QueryOptions) -> Result<Vec<Customer>,PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id,query_ops).await
    }

    pub async fn add_customer(&mut self, item : Customer) -> Result<Vec<Customer>,PlatypusError> {
        self.persist.as_mut().unwrap().create_tmf_item(item).await
    }
}