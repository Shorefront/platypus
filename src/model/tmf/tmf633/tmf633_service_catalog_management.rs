//! Service Catalog Management API Module

use tmflib::tmf633::{
    service_candidate::ServiceCandidate, service_catalog::ServiceCatalog,
    service_category::ServiceCategory, service_specification::ServiceSpecification,
};

use crate::QueryOptions;

use crate::common::{error::PlatypusError, persist::Persistence};

#[derive(Clone, Default, Debug)]
pub struct TMF633ServiceCatalogManagement {
    persist: Option<Persistence>,
}

impl TMF633ServiceCatalogManagement {
    pub fn new() -> TMF633ServiceCatalogManagement {
        TMF633ServiceCatalogManagement { persist: None }
    }
    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    // Create operations
    pub async fn create_candidate(
        &self,
        item: ServiceCandidate,
    ) -> Result<Vec<ServiceCandidate>, PlatypusError> {
        self.persist.as_ref().unwrap().create_tmf_item(item).await
    }

    pub async fn create_catalog(
        &self,
        item: ServiceCatalog,
    ) -> Result<Vec<ServiceCatalog>, PlatypusError> {
        self.persist.as_ref().unwrap().create_tmf_item(item).await
    }

    pub async fn create_category(
        &self,
        item: ServiceCategory,
    ) -> Result<Vec<ServiceCategory>, PlatypusError> {
        self.persist.as_ref().unwrap().create_tmf_item(item).await
    }

    pub async fn create_specification(
        &self,
        item: ServiceSpecification,
    ) -> Result<Vec<ServiceSpecification>, PlatypusError> {
        self.persist.as_ref().unwrap().create_tmf_item(item).await
    }

    // List operations
    pub async fn get_candidates(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<ServiceCandidate>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }
    pub async fn get_catalogs(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<ServiceCatalog>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_categories(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<ServiceCategory>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_specifications(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<ServiceSpecification>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    // Get Operations
    pub async fn get_candidate(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<ServiceCandidate>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    pub async fn get_catalog(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<ServiceCatalog>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    pub async fn get_category(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<ServiceCategory>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    pub async fn get_specification(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<ServiceSpecification>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    // Update Operations
    pub async fn update_candidate(
        &self,
        id: String,
        patch: ServiceCandidate,
    ) -> Result<Vec<ServiceCandidate>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item(id, patch)
            .await
    }

    pub async fn update_specification(
        &self,
        id: String,
        patch: ServiceSpecification,
    ) -> Result<Vec<ServiceSpecification>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item(id, patch)
            .await
    }

    pub async fn update_category(
        &self,
        id: String,
        patch: ServiceCategory,
    ) -> Result<Vec<ServiceCategory>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item(id, patch)
            .await
    }

    pub async fn update_catalog(
        &self,
        id: String,
        patch: ServiceCatalog,
    ) -> Result<Vec<ServiceCatalog>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item(id, patch)
            .await
    }

    // Delete Operations
    pub async fn delete_candidate(&self, id: String) -> Result<ServiceCandidate, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<ServiceCandidate>(id)
            .await
    }
    pub async fn delete_catalog(&self, id: String) -> Result<ServiceCatalog, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<ServiceCatalog>(id)
            .await
    }
    pub async fn delete_category(&self, id: String) -> Result<ServiceCategory, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<ServiceCategory>(id)
            .await
    }
    pub async fn delete_specification(
        &self,
        id: String,
    ) -> Result<ServiceSpecification, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<ServiceSpecification>(id)
            .await
    }
}
