//! TMF620 Catalog Management Module

use tmflib::tmf620::category::Category;
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_specification::ProductSpecification;

use serde::{Deserialize,Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
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

    pub async fn add_category(&mut self, category : Category) -> Result<String,surrealdb::Error> {
        //self.categories.push(category);

        // Also push into db
        let records : Vec<Record> = self.db.create("category").await?;
        dbg!(records);
        Ok(format!("Category {} added",category.name.as_ref().unwrap()).into())
    }
}