//! TMF645 Service Qualification Management Module

use crate::QueryOptions;

use tmflib::tmf645::check_service_qualification::CheckServiceQualification;

use crate::common::error::PlatypusError;
use crate::common::persist::Persistence;

pub struct TMF645ServiceQualificationManagement {
    persist : Option<Persistence>,
}

impl TMF645ServiceQualificationManagement {
    pub fn new(persist: Option<Persistence>) -> TMF645ServiceQualificationManagement {
        TMF645ServiceQualificationManagement {
            persist,
        }
    }

    pub fn persist(&mut self, persist : Persistence) {
        self.persist = Some(persist);
    }    

    pub async fn add_check_qualification(&self, item : CheckServiceQualification) -> Result<Vec< CheckServiceQualification>,PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .create_tmf_item(item.clone()).await;
        #[cfg(feature = "events")]
        {
            // No events on TMF645 yet.
        }
        result
    }

    pub async fn get_check_qualifications(&self,query_ops : QueryOptions) -> Result<Vec<CheckServiceQualification>,PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_ops).await
    }

    pub async fn get_check_qualification(&self, id : String, query_ops : QueryOptions) -> Result<Vec<CheckServiceQualification>,PlatypusError> {
        self.persist.as_ref().unwrap().get_item(id,query_ops).await
    }

    pub async fn update_check_qualification(&self, id : String, patch : CheckServiceQualification) -> Result<Vec<CheckServiceQualification>,PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .patch_tmf_item(id, patch)
            .await;
        #[cfg(feature = "events")]
        {

        }
        result
    }

    pub async fn delete_check_qualification(&self, id : String) -> Result<CheckServiceQualification,PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .delete_tmf_item(id)
            .await;
        #[cfg(feature = "events")]
        {

        }
        result
    }

}