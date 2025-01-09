use crate::common::persist::Persistence;


#[derive(Clone,Default,Debug)]
pub struct TMF633ServiceCatalogManagement {
    persist : Option<Persistence>,
}

impl TMF633ServiceCatalogManagement {
    pub fn new() -> TMF633ServiceCatalogManagement {
        TMF633ServiceCatalogManagement {
            persist: None
        }
    }
    pub fn persist(&mut self, persist: Persistence) {
        self.persist = Some(persist);
    }
}