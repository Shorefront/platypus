//! Component Template
//!

use super::TEMPLATE_CATEGORY;
use serde::{Deserialize, Serialize};
use std::convert::Into;
use tmflib::tmf620::{
    category::{Category, CategoryRef},
    product_offering::ProductOffering,
};

/// A Component Template defines how to build a component
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ComponentTemplate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ProductOffering>,
}

impl ComponentTemplate {
    pub fn new(name: impl Into<String>) -> ComponentTemplate {
        ComponentTemplate::default().name(name)
    }

    pub fn name(mut self, name: impl Into<String>) -> ComponentTemplate {
        self.name = name.into();
        let cat = Category::new(TEMPLATE_CATEGORY.to_string());
        let offering =
            ProductOffering::new(self.name.clone()).with_category(CategoryRef::from(&cat));
        self.component = Some(offering);
        self
    }
}

impl Into<ProductOffering> for ComponentTemplate {
    fn into(self) -> ProductOffering {
        self.component.unwrap()
    }
}
