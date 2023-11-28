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
        // Need to handle customer code collisions here.
        // Step1, try to retrieve a customer record using customer.code
        // Step2, if found, generate a new code with an offset. Rinse and repeat until no collision.
        //let result = self.persist.get_items_filter("item.code={}", query_opts)
        // SQL = "SELECT * FROM customer WHERE item.characteristic.name == 'code'"
        self.persist.create_tmf_item(customer).await
    }

    pub async fn get_customers(&self,query_opts : QueryOptions) -> Result<Vec<Customer>,PlatypusError> {
        self.persist.get_tmf_items(query_opts).await
    }

    pub async fn get_customer(&self, id : String, query_opts : QueryOptions) -> Result<Vec<Customer>,PlatypusError> {
        self.persist.get_item(id,query_opts).await
    }
}