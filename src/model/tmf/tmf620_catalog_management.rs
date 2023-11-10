//! TMF620 Catalog Management Module

use tmflib::tmf620::category::{Category, CategoryRef};
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_offering_price::ProductOfferingPrice;
use tmflib::tmf620::product_specification::ProductSpecification;

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

    pub async fn get_catalogs(&self, fields : Option<Vec<String>>) -> Result<Vec<Catalog>,PlatypusError> {
        self.persist.get_items(fields).await
    }

    pub async fn get_categories(&self, fields : Option<Vec<String>>) -> Result<Vec<Category>,PlatypusError> {
        // Get all category records
        self.persist.get_items(fields).await
    }

    pub async fn get_specifications(&self, fields : Option<Vec<String>>) -> Result<Vec<ProductSpecification>,PlatypusError> {
        // Get all specifications
        self.persist.get_items(fields).await
    }

    pub async fn get_specification(&self, id : String, fields : Option<Vec<String>>) -> Result<Vec<ProductSpecification>,PlatypusError> {
        self.persist.get_item(id,fields).await
    }

    pub async fn get_offers(&self, fields : Option<Vec<String>>) -> Result<Vec<ProductOffering>,PlatypusError> {
        self.persist.get_items(fields).await  
    }

    pub async fn get_offer(&self, id : String, fields : Option<Vec<String>>) -> Result<Vec<ProductOffering>,PlatypusError> {
        match fields {
            Some(f) => self.persist.get_tmf_item_fields(id, f).await,
            None => self.persist.get_tmf_item(id).await
        }
    }

    pub async fn get_prices(&self, fields : Option<Vec<String>>) -> Result<Vec<ProductOfferingPrice>,PlatypusError> {
        match fields {
            Some(f) => self.persist.get_tmf_items_fields(f).await,
            None => self.persist.get_tmf_items().await
        }
    }

    pub async fn get_price(&self, id : String, fields : Option<Vec<String>>) -> Result<Vec<ProductOfferingPrice>,PlatypusError> {
        match fields {
            Some(f) => self.persist.get_tmf_item_fields(id, f).await,
            None => self.persist.get_tmf_item(id).await
        }
    }

    pub async fn get_category(&self,id : String, _fields : Option<Vec<String>>) -> Result<Option<Category>,PlatypusError> {
        //let output : Vec<CategoryRecord>  = self.db.select("catagory").range(id(id)).await.unwrap();
        //let name : &str = "Root";
        let query = format!("SELECT * FROM category:{}",id);
        let mut output = self.persist.db.query(query).await?;
        let result : Vec<CategoryRecord> = output.take(0)?;
        let mut cat = result.first().cloned().map(|cat| {
            cat.category
        });
        // Now enrich with any records where parentId = id
        let sub_query = format!("SELECT * FROM category where category.parentId = '{}'",id);
        let mut response = self.persist.db.query(sub_query).await?;
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

    pub async fn get_catalog(&self, id : String, fields : Option<Vec<String>>) -> Result<Vec<Catalog>,PlatypusError>  {
        match fields {
            Some(f) => self.persist.get_tmf_item_fields(id, f).await,
            None => self.persist.get_tmf_item(id).await
        }
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