//! TMF620 Catalog Management Module

use tmflib::tmf620::category::{Category, CategoryRef};
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use tmflib::tmf620::product_specification::ProductSpecification;
use tmflib::HasId;
use super::{store_tmf_item,get_tmf_item,get_tmf_items};

use serde::{Deserialize,Serialize};

use log::error;

use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;

use crate::common::error::PlatypusError;
use super::TMF;

#[derive(Debug,Clone)]
pub struct TMF620CatalogManagement {
    // Use of vectors here is very simplistic, ideally need a hash.
    db : Surreal<Db>,
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
    pub fn new(db : Surreal<Db>) -> TMF620CatalogManagement {
        TMF620CatalogManagement { 
            db,
            categories: vec![], 
            catalogs: vec![],
            offers: vec![],
            specifications: vec![],
        }
    }

    pub async fn add_catalog(&mut self, catalog : Catalog) -> Result<Vec<Catalog>,PlatypusError> {
        store_tmf_item(self.db.clone(), catalog).await
    }

    pub async fn add_specification(&mut self, mut specification: ProductSpecification) -> Result<Vec<ProductSpecification>,PlatypusError> {
        // New record, needs appropriate status
        specification.status("New");
        store_tmf_item(self.db.clone(), specification).await
    }

    pub async fn add_offering(&mut self, mut offering : ProductOffering) -> Result<Vec<ProductOffering>,PlatypusError> {
        offering.status("New");
        store_tmf_item(self.db.clone(), offering).await
    }

    pub async fn add_price(&mut self, price: ProductOfferingPrice) -> Result<Vec<ProductOfferingPrice>,PlatypusError> {
        store_tmf_item(self.db.clone(),price).await
    }

    pub async fn add_category(&mut self, mut category : Category) -> Result<Vec<Category>,PlatypusError> {
        
        if !category.root() && category.parent_id.is_some() {
            let parent_id = category.parent_id.as_ref().unwrap().clone();
            // Need to check if parentId is pointing to a valid parent
            let parent_query = format!("SELECT * FROM category:{}",parent_id);
            let mut parent_resp = self.db.query(parent_query).await?;
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

        store_tmf_item(self.db.clone(), category).await
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>,PlatypusError> {
        // Get all category records
        let get_records : Vec<CategoryRecord> = self.db.select("category").await?;
        let mut output : Vec<Category> = vec![];
        
        // Need to generate a vec of sub_categories
        get_records.iter().for_each(|cat| {
            output.push(cat.category.clone());
        });
        Ok(output)
    }

    pub async fn get_specifications(&self) -> Result<Vec<ProductSpecification>,PlatypusError> {
        // Get all specifications
        get_tmf_items(self.db.clone()).await
    }

    pub async fn get_specification(&self, id : String) -> Result<Vec<ProductSpecification>,PlatypusError> {
        get_tmf_item(self.db.clone(),id).await
    }

    pub async fn get_offers(&self) -> Result<Vec<ProductOffering>,PlatypusError> {
        get_tmf_items(self.db.clone()).await
    }

    pub async fn get_offer(&self, id : String) -> Result<Vec<ProductOffering>,PlatypusError> {
        get_tmf_item(self.db.clone(),id).await
    }

    pub async fn get_prices(&self) -> Result<Vec<ProductOfferingPrice>,PlatypusError> {
        get_tmf_items(self.db.clone()).await
    }

    pub async fn get_price(&self, id : String) -> Result<Vec<ProductOfferingPrice>,PlatypusError> {
        get_tmf_item(self.db.clone(),id).await
    }

    pub async fn get_category(&self,id : String) -> Result<Option<Category>,PlatypusError> {
        //let output : Vec<CategoryRecord>  = self.db.select("catagory").range(id(id)).await.unwrap();
        //let name : &str = "Root";
        let query = format!("SELECT * FROM category:{}",id);
        let mut output = self.db.query(query).await?;
        let result : Vec<CategoryRecord> = output.take(0)?;
        let mut cat = result.first().cloned().map(|cat| {
            cat.category
        });
        // Now enrich with any records where parentId = id
        let sub_query = format!("SELECT * FROM category where category.parentId = '{}'",id);
        let mut response = self.db.query(sub_query).await?;
        let vec : Vec<CategoryRecord> = response.take(0)?;
        let mut sub_category : Vec<CategoryRef> = vec![];
        vec.iter().for_each(|cr| {
            // Take each category record and
            // Extract the category
            // Convert to CategoryRef
            // Add to Vec
            let cat = cr.clone().category;
            let cat_ref = CategoryRef::from(&cat);
            sub_category.push(cat_ref);
        });

        // Now enrich with offers that have parentId = id
        
        cat.as_mut().unwrap().sub_category = Some(sub_category);
        Ok(cat)
    }
}