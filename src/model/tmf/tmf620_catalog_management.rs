//! TMF620 Catalog Management Module

use tmflib::tmf620::category::Category;
use tmflib::tmf620::catalog::Catalog;

pub struct TMF620CatalogManagement {
    pub categories: Vec<Category>,
    pub catalogs: Vec<Catalog>,
}

impl TMF620CatalogManagement {
    pub fn new() -> TMF620CatalogManagement {
        TMF620CatalogManagement { 
            categories: vec![], 
            catalogs: vec![] 
        }
    }
}