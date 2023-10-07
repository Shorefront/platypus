//! TMF620 Catalog Management Module

use tmflib::tmf620::category::{Category, CategoryRef};
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_specification::ProductSpecification;

use serde::{Deserialize,Serialize};

use log::error;

use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;

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
struct CategoryRecord {
    #[allow(dead_code)]
    id: Option<Thing>,
    category : Category,
}

impl TMF620CatalogManagement {
    pub fn new(db : Surreal<Db>) -> TMF620CatalogManagement {
        TMF620CatalogManagement { 
            db,
            categories: vec![], 
            catalogs: vec![],
            offers: vec![],
            specifications: vec![],
        }
    }

    pub async fn add_category(&mut self, mut category : Category) -> Result<String,surrealdb::Error> {
        
        if !category.is_root && category.parent_id.is_some() {
            let parent_id = category.parent_id.as_ref().unwrap().clone();
            // Need to check if parentId is pointing to a valid parent
            let parent_query = format!("SELECT * FROM category:{}",parent_id);
            let mut parent_resp = self.db.query(parent_query).await?;
            let parent : Vec<CategoryRecord> = parent_resp.take(0).unwrap();
            if parent.len() == 0 {
                // Throw error, parent not found
                error!("ParentId {} not found for child {}",&parent_id,category.id.clone().unwrap());
            }
        }

        // Simiarly, if flagged as root, cannot also have parent_id
        if category.is_root {
            category.parent_id = None;
        }

        // Also push into db
        let record = CategoryRecord {
            id : Some(Thing {
                tb: "category".into(),
                id: category.id.clone().unwrap().into(),
            }),
            category: category.clone(),
        };
        let _insert_records : Vec<CategoryRecord> = self.db.create("category").content(record).await?;

        Ok(format!("Category added").to_string())
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>,surrealdb::Error> {
        // Get all category records
        let get_records : Vec<CategoryRecord> = self.db.select("category").await?;
        let mut output : Vec<Category> = vec![];
        
        // Need to generate a vec of sub_categories
        get_records.iter().for_each(|cat| {
            output.push(cat.category.clone());
        });
        Ok(output)
    }

    pub async fn get_category(&self, id : String) -> Result<Option<Category>,surrealdb::Error> {
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
        
        cat.as_mut().unwrap().sub_category = Some(sub_category);
        Ok(cat)
    }
}