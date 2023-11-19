//! TMF620 Catalog Management Module

use tmflib::tmf620::category::{Category, CategoryRef};
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use tmflib::tmf620::product_specification::ProductSpecification;

use crate::QueryOptions;

use serde::{Deserialize,Serialize};

use log::error;

use surrealdb::sql::Thing;

use crate::common::error::PlatypusError;
use crate::common::persist::Persistence;

#[derive(Debug,Clone)]
pub struct TMF620CatalogManagement {
    // Use of vectors here is very simplistic, ideally need a hash.
    //db : Surreal<Db>,
    persist : Persistence,
    pub categories: Vec<Category>,
    pub catalogs: Vec<Catalog>,
    pub offers: Vec<ProductOffering>,
    pub specifications: Vec<ProductSpecification>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CatalogRecord {
    #[allow(dead_code)]
    id: Option<Thing>,
    catalog: Catalog,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CategoryRecord {
    #[allow(dead_code)]
    id: Option<Thing>,
    category : Category,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GenericRecord<T> {
    #[allow(dead_code)]
    id: Option<Thing>,
    item : T,
}

impl TMF620CatalogManagement 
    {
    pub fn new(persist : Persistence) -> TMF620CatalogManagement {
        TMF620CatalogManagement { 
            persist,
            categories: vec![], 
            catalogs: vec![],
            offers: vec![],
            specifications: vec![],
        }
    }

    pub async fn add_catalog(&mut self, catalog : Catalog) -> Result<Vec<Catalog>,PlatypusError> {
        self.persist.create_tmf_item(catalog).await
    }

    pub async fn add_specification(&mut self, mut specification: ProductSpecification) -> Result<Vec<ProductSpecification>,PlatypusError> {
        // New record, needs appropriate status
        specification.status("New");
        self.persist.create_tmf_item(specification).await
    }

    pub async fn add_offering(&mut self, mut offering : ProductOffering) -> Result<Vec<ProductOffering>,PlatypusError> {
        offering.status("New");
        self.persist.create_tmf_item(offering).await
    }

    pub async fn add_price(&mut self, price: ProductOfferingPrice) -> Result<Vec<ProductOfferingPrice>,PlatypusError> {
        self.persist.create_tmf_item(price).await
    }

    pub async fn add_category(&mut self, mut category : Category) -> Result<Vec<Category>,PlatypusError> {
        
        if !category.root() && category.parent_id.is_some() {
            let parent_id = category.parent_id.as_ref().unwrap().clone();
            // Need to check if parentId is pointing to a valid parent
            let parent_query = format!("SELECT * FROM category:{}",parent_id);
            let mut parent_resp = self.persist.db.query(parent_query).await?;
            let parent : Vec<CategoryRecord> = parent_resp.take(0).unwrap();
            if parent.len() == 0 {
                // Throw error, parent not found
                let msg = format!("ParentId {} not found for child {}",&parent_id,category.id.clone().unwrap());
                error!("add_category: {msg}");
                return Err(PlatypusError { message: msg, })
            }
        }

        // Simiarly, if flagged as root, cannot also have parent_id
        if category.root() {
            category.parent_id = None;
        }

        self.persist.create_tmf_item(category).await
    }

    pub async fn get_catalogs(&self, query_opts : QueryOptions) -> Result<Vec<Catalog>,PlatypusError> {
        self.persist.get_items(query_opts).await
    }

    pub async fn get_categories(&self, query_opts : QueryOptions) -> Result<Vec<Category>,PlatypusError> {
        // Get all category records
        self.persist.get_items(query_opts).await
    }

    pub async fn get_specifications(&self, query_opts : QueryOptions) -> Result<Vec<ProductSpecification>,PlatypusError> {
        // Get all specifications
        self.persist.get_items(query_opts).await
    }

    pub async fn get_specification(&self, id : String, query_opts : QueryOptions) -> Result<Vec<ProductSpecification>,PlatypusError> {
        self.persist.get_item(id,query_opts).await
    }

    pub async fn get_offers(&self, query_opts : QueryOptions) -> Result<Vec<ProductOffering>,PlatypusError> {
        self.persist.get_items(query_opts).await  
    }

    pub async fn get_offer(&self, id : String, query_opts : QueryOptions) -> Result<Vec<ProductOffering>,PlatypusError> {
        self.persist.get_item(id,query_opts).await
    }

    pub async fn get_prices(&self, query_opts : QueryOptions) -> Result<Vec<ProductOfferingPrice>,PlatypusError> {
        self.persist.get_items(query_opts).await
    }

    pub async fn get_price(&self, id : String, query_opts : QueryOptions) -> Result<Vec<ProductOfferingPrice>,PlatypusError> {
        self.persist.get_item(id,query_opts).await
    }

    pub async fn get_child_category(&self, parent_id : String, query_opts : QueryOptions) -> Result<Vec<Category>,PlatypusError> {
        // Look for categories with common parent_id
        self.persist.get_items_filter(format!("item.parent_id = {}",parent_id), query_opts).await
    }

    pub async fn get_category(&self,id : String,query_opts : QueryOptions) -> Result<Vec<Category>,PlatypusError> {
        let result : Vec<Category> = self.persist.get_item(id,query_opts.clone()).await?;
        let mut first = result.first().as_mut().unwrap().clone();
        let parent_id = first.id.clone().unwrap();
        let children = self.get_child_category(parent_id, query_opts).await?;
        // Map through children converting to CategoryRef and appending onto cat
        let mut kids : Vec<CategoryRef> = children.into_iter().map(|c| {
            CategoryRef::from(&c)
        }).collect();
        first.sub_category.as_mut().unwrap().append(&mut kids);
        
        Ok(vec![first.clone()])
    }

    pub async fn get_catalog(&self, id : String, query_opts : QueryOptions) -> Result<Vec<Catalog>,PlatypusError>  {
        self.persist.get_item(id,query_opts).await
    }
    
    pub async fn patch_specification(&self, id : String, patch : String) -> Result<Vec<ProductSpecification>,PlatypusError> {
        self.persist.patch_tmf_item(id, patch).await
    }

    pub async fn patch_offering(&self, id : String, patch : String) -> Result<Vec<ProductOffering>, PlatypusError> {
        self.persist.patch_tmf_item(id, patch).await
    }
    
    pub async fn patch_price(&self, id : String, patch : String) -> Result<Vec<ProductOfferingPrice>, PlatypusError> {
        self.persist.patch_tmf_item(id,patch).await
    }

    pub async fn delete_specification(&self, id : String) -> Result<bool,PlatypusError> {
        self.persist.delete_tmf_item::<ProductSpecification>(id).await
    }

    pub async fn delete_offering(&self, id : String) -> Result<bool, PlatypusError> {
        self.persist.delete_tmf_item::<ProductOffering>(id).await
    }
    
    pub async fn delete_price(&self, id : String) -> Result<bool, PlatypusError> {
        self.persist.delete_tmf_item::<ProductOfferingPrice>(id).await
    }
}