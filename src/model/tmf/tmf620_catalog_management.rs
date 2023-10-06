//! TMF620 Catalog Management Module

use std::process::id;

use tmflib::tmf620::category::Category;
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::product_specification::ProductSpecification;

use serde::{Deserialize,Serialize};

use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;

use log::{error,info,debug};

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

    pub async fn add_category(&mut self, category : Category) -> Result<String,surrealdb::Error> {
        //self.categories.push(category);

        // Also push into db
        let record = CategoryRecord {
            id : Some(Thing {
                tb: "category".into(),
                id: category.id.clone().unwrap().into(),
            }),
            category,
        };
        let _insert_records : Vec<CategoryRecord> = self.db.create("category").content(record).await?;

        Ok(format!("Category added").into())
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>,surrealdb::Error> {
        // Get all category records
        let get_records : Vec<CategoryRecord> = self.db.select("category").await?;
        let mut output : Vec<Category> = vec![];
        dbg!(&get_records);
        // Need to generate a vec of sub_categories
        get_records.iter().for_each(|cat| {
            output.push(cat.category.clone());
        });
        Ok(output)
    }

    pub async fn get_category(&self, id : String) -> Result<Option<Category>,surrealdb::Error> {
        //let output : Vec<CategoryRecord>  = self.db.select("catagory").range(id(id)).await.unwrap();
        //let name : &str = "Root";
        let query = format!("SELECT * FROM category WHERE category.id = '{}'",id);
        let mut output = self.db.query(query).await?;
        let result : Vec<CategoryRecord> = output.take(0)?;
        let cat = result.first().cloned().map(|cat| {
            cat.category
        });
        dbg!(&cat);
        Ok(cat)
    }
}