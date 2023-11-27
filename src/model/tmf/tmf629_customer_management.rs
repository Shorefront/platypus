//! Customer Management Module
//! 

use tmflib::HasId;
use tmflib::tmf629::customer::Customer;
use crate::common::{error::PlatypusError, persist::Persistence};

use super::{tmf_payload,TMF};

use crate::QueryOptions;

use log::{debug,error};

#[derive(Clone, Debug)]
pub struct TMF629CustomerManagement {
    persist : Persistence,
}

impl TMF629CustomerManagement {
    pub fn new(persist : Persistence) -> TMF629CustomerManagement {
        TMF629CustomerManagement { persist }
    }

    pub async fn add_customer(&mut self, customer : Customer) -> Result<Vec<Customer>,PlatypusError> {
        self.persist.create_tmf_item(customer).await
    }

    pub async fn get_customers(&self,query_opts : QueryOptions) -> Result<Vec<Customer>,PlatypusError> {
        self.persist.get_tmf_items(query_opts).await
    }

    pub async fn get_customer(&self, id : String, query_opts : QueryOptions) -> Result<Vec<Customer>,PlatypusError> {
        self.persist.get_item(id,query_opts).await
    }
}