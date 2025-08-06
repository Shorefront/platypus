//! TMF620 Catalog Management Module

use tmflib::common::event::EventPayload;
use tmflib::tmf620::catalog::{Catalog, CatalogEventType};
use tmflib::tmf620::category::Category;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use tmflib::tmf620::product_specification::ProductSpecification;

use crate::QueryOptions;

use serde::{Deserialize, Serialize};

use surrealdb::sql::Thing;

use crate::common::error::PlatypusError;
use crate::common::persist::Persistence;

#[derive(Debug, Clone)]
pub struct TMF620CatalogManagement {
    // Use of vectors here is very simplistic, ideally need a hash.
    //db : Surreal<Db>,
    persist: Option<Persistence>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CategoryRecord {
    #[allow(dead_code)]
    id: Option<Thing>,
    category: Category,
}

impl TMF620CatalogManagement {
    pub fn new(persist: Option<Persistence>) -> TMF620CatalogManagement {
        TMF620CatalogManagement { persist }
    }

    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn add_catalog(&mut self, catalog: Catalog) -> Result<Vec<Catalog>, PlatypusError> {
        let result = self
            .persist
            .as_mut()
            .unwrap()
            .create_tmf_item_lastupdate(catalog.clone())
            .await;
        #[cfg(feature = "events")]
        {
            let event = catalog.to_event(CatalogEventType::CatalogCreateEvent);
            let _new_event = self
                .persist
                .as_ref()
                .unwrap()
                .store_tmf_event(event)
                .await?;
        }
        result
    }

    pub async fn add_specification(
        &mut self,
        mut specification: ProductSpecification,
    ) -> Result<Vec<ProductSpecification>, PlatypusError> {
        // New record, needs appropriate status
        specification.status("New");
        self.persist
            .as_mut()
            .unwrap()
            .create_tmf_item_lastupdate(specification)
            .await
    }

    pub async fn add_offering(
        &mut self,
        mut offering: ProductOffering,
    ) -> Result<Vec<ProductOffering>, PlatypusError> {
        offering.status("New");
        self.persist
            .as_mut()
            .unwrap()
            .create_tmf_item_lastupdate(offering)
            .await
    }

    pub async fn add_price(
        &mut self,
        price: ProductOfferingPrice,
    ) -> Result<Vec<ProductOfferingPrice>, PlatypusError> {
        self.persist
            .as_mut()
            .unwrap()
            .create_tmf_item_lastupdate(price)
            .await
    }

    pub async fn add_category(
        &mut self,
        mut category: Category,
    ) -> Result<Vec<Category>, PlatypusError> {
        // If flagged as root, cannot also have parent_id
        if category.root() {
            category.parent_id = None;
        }

        self.persist
            .as_mut()
            .unwrap()
            .create_tmf_item_lastupdate(category)
            .await
    }

    pub async fn get_catalogs(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<Catalog>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_categories(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<Category>, PlatypusError> {
        // Get all category records
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_specifications(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<ProductSpecification>, PlatypusError> {
        // Get all specifications
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_specification(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<ProductSpecification>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    pub async fn get_offers(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<ProductOffering>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_offer(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<ProductOffering>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    pub async fn get_prices(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<ProductOfferingPrice>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_price(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<ProductOfferingPrice>, PlatypusError> {
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
    ) -> Result<Vec<Category>, PlatypusError> {
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
    ) -> Result<Vec<Catalog>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    pub async fn patch_category(
        &self,
        id: String,
        patch: Category,
    ) -> Result<Vec<Category>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item_lastupdate(id, patch)
            .await
    }

    pub async fn patch_catalog(
        &self,
        id: String,
        patch: Catalog,
    ) -> Result<Vec<Catalog>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item_lastupdate(id, patch)
            .await
    }

    pub async fn patch_specification(
        &self,
        id: String,
        patch: ProductSpecification,
    ) -> Result<Vec<ProductSpecification>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item_lastupdate(id, patch)
            .await
    }

    pub async fn patch_offering(
        &self,
        id: String,
        patch: ProductOffering,
    ) -> Result<Vec<ProductOffering>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item_lastupdate(id, patch)
            .await
    }

    pub async fn patch_price(
        &self,
        id: String,
        patch: ProductOfferingPrice,
    ) -> Result<Vec<ProductOfferingPrice>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .patch_tmf_item_lastupdate(id, patch)
            .await
    }

    pub async fn delete_category(&self, id: String) -> Result<Category, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<Category>(id)
            .await
    }

    pub async fn delete_catalog(&self, id: String) -> Result<Catalog, PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<Catalog>(id)
            .await;
        #[cfg(feature = "events")]
        {
            if result.is_ok() {
                let catalog = result.as_ref().ok().unwrap();
                let event = catalog.to_event(CatalogEventType::CatalogDeleteEvent);
                let _new_event = self
                    .persist
                    .as_ref()
                    .unwrap()
                    .store_tmf_event(event)
                    .await?;
            }
        }
        result
    }

    pub async fn delete_specification(
        &self,
        id: String,
    ) -> Result<ProductSpecification, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<ProductSpecification>(id)
            .await
    }

    pub async fn delete_offering(&self, id: String) -> Result<ProductOffering, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<ProductOffering>(id)
            .await
    }

    pub async fn delete_price(&self, id: String) -> Result<ProductOfferingPrice, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .delete_tmf_item::<ProductOfferingPrice>(id)
            .await
    }
}
