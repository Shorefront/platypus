/// Shopping Cart Module
///
use crate::common::{error::PlatypusError, persist::Persistence};
use crate::QueryOptions;
use tmflib::tmf663::shopping_cart::ShoppingCart;

#[derive(Clone, Debug)]
pub struct TMF663ShoppingCartManagement {
    persist: Option<Persistence>,
}

impl TMF663ShoppingCartManagement {
    pub fn new(persist: Option<Persistence>) -> TMF663ShoppingCartManagement {
        TMF663ShoppingCartManagement { persist }
    }

    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }

    pub async fn add_cart(&self, item: ShoppingCart) -> Result<Vec<ShoppingCart>, PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .create_tmf_item(item.clone())
            .await;
        // #[cfg(feature = "events")]
        // {
        //     let event = item.to_event();
        //     let _ = self.persist.as_ref().unwrap().store_tmf_event(event).await?;
        // }
        result
    }

    pub async fn get_carts(
        &self,
        query_opts: QueryOptions,
    ) -> Result<Vec<ShoppingCart>, PlatypusError> {
        self.persist.as_ref().unwrap().get_items(query_opts).await
    }

    pub async fn get_cart(
        &self,
        id: String,
        query_opts: QueryOptions,
    ) -> Result<Vec<ShoppingCart>, PlatypusError> {
        self.persist
            .as_ref()
            .unwrap()
            .get_item(id, query_opts)
            .await
    }

    pub async fn update_cart(
        &self,
        id: String,
        patch: ShoppingCart,
    ) -> Result<Vec<ShoppingCart>, PlatypusError> {
        let result = self
            .persist
            .as_ref()
            .unwrap()
            .patch_tmf_item(id, patch.clone())
            .await;
        // #[cfg(feature = "events")]
        // {
        //     let event = match patch.status.is_some() {
        //         true => patch.to_event(),
        //         false => patch.to_event(),
        //     };
        //     let _ = self.persist.as_ref().unwrap().store_tmf_event(event).await?;
        // }
        result
    }

    pub async fn delete_cart(&self, id: String) -> Result<ShoppingCart, PlatypusError> {
        let result = self.persist.as_ref().unwrap().delete_tmf_item(id).await;
        // #[cfg(feature = "events")]
        // {
        //     let event = ShoppingCart::new(id).to_event();
        //     let _ = self.persist.as_ref().unwrap().store_tmf_event(event).await?;
        // }
        result
    }
}
