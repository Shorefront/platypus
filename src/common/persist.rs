//! Persistence Module
//! 
use surrealdb::{engine::any::Any, opt::auth::Root};
use surrealdb::{RecordId, Surreal};
// use surrealdb::Surreal::Root;

use log::{info,debug};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
#[cfg(feature = "events")]
use tmflib::common::event::Event;

use crate::QueryOptions;
use super::config::Config;
use super::error::PlatypusError;
use tmflib::{HasId, HasLastUpdate};

/// Generic TMF struct for DB
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TMF<T> {
    pub id : RecordId,
    pub item : T,
}

#[derive(Clone,Debug)]
pub struct Persistence {
    pub db : Surreal<Any>,
}

impl Persistence {
    pub async fn new(config : &Config) -> Result<Persistence,PlatypusError> {
        use surrealdb::engine::any;

        // Connect to the database
        let db_host = config.get("DB_HOST").ok_or(PlatypusError::from("DB_HOST not defined"))?;
        let db_ns   = config.get("DB_NS").ok_or(PlatypusError::from("DB Namespace not configured"))?;
        let db_user = config.get("DB_USER").ok_or(PlatypusError::from("DB User not set"))?;
        let db_pass = config.get("DB_PASS").ok_or(PlatypusError::from("DB Pass not set"))?;

        let db = any::connect(db_host).await
            .expect("Could not connect");

        // Select a namespace and database
        db.use_ns(db_ns).use_db("platypus-db").await
            .expect("Could not set namespace");

                // Authenticate
        db.signin(Root {
            username: db_user.as_str(),
            password: db_pass.as_str(),
        }).await
            .expect("Could not authenticate");

        Ok(Persistence { 
            db,
        })
    }

    fn tmf_payload<'a, T : HasId + Serialize + Clone + Deserialize<'a>>(item : T) -> TMF<T> {
        TMF {
            id : (T::get_class(),item.get_id()).into(),
            item
        }
    }

    fn query_to_filter(query_opts: QueryOptions) -> String {
        // Attempt to convert a QueryOption into a WHERE clause
        match query_opts.name {
            Some(f) => format!("WHERE item.name = '{}'",f),
            None => String::new(),
        }
    }

    fn query_to_fields(query_opts : QueryOptions) -> Option<Vec<String>> {
        match query_opts.fields {
            Some(f) => {
                // Detect a 'none' case
                let mut output : Vec<String> = vec![];
                if f == "none" || f.is_empty() {
                    return Some(output);
                };
                f.split(',').for_each(|f| {
                    output.push(f.to_owned());
                });
                Some(output)
            },
            None => None,
        }
    }

    /// List objects of a particular type
    pub async fn get_tmf_items<T : HasId + Serialize + Clone + DeserializeOwned>(&self,query_opts : QueryOptions) -> Result<Vec<T>,PlatypusError> {

        let filter = Persistence::query_to_filter(query_opts.clone());

        let limit = match query_opts.limit {
            Some(l) => format!("LIMIT BY {}",l),
            None => String::new(),
        };

        let offset = match query_opts.offset {
            Some(o) => format!("START AT {}",o),
            None => String::new(),
        };

        let query = format!("SELECT * FROM {} {} {} {}",T::get_class(),filter,limit,offset);
        let mut output = self.db.query(query).await?;
        let result : Vec<TMF<T>> = output.take(0)?;
        let item = result.iter().map(|tmf| {
            tmf.clone().item
        }).collect();
        Ok(item)    
    }

    pub async fn get_tmf_items_fields<T : HasId + Serialize + Clone + DeserializeOwned>(&self, query_opts : QueryOptions) -> Result<Vec<T>,PlatypusError> {
        // Generate additional fields from vec
        let field_query = match Persistence::query_to_fields(query_opts.clone()) {
            Some(f) => {
                let fields : Vec<String> = f.into_iter().map(|f| {
                    // Standard payload has TMF payload under 'item' object thus need to prepend 'item' to each field.
                    format!("item.{f}")
                }).collect();
                format!(",{}",fields.join(","))
            },
            None => {
                String::new()
            }
        };
        let filter = Persistence::query_to_filter(query_opts.clone());

        let limit = match query_opts.limit {
            Some(l) => format!("LIMIT BY {}",l),
            None => String::new(),
        };

        let offset = match query_opts.offset {
            Some(o) => format!("START AT {}",o),
            None => String::new(),
        };
        
        let query = format!("SELECT item.id, item.href {} FROM {} {} {} {}",field_query, T::get_class(),filter,limit,offset);
        let mut output = self.db.query(query).await?;
        let result : Vec<TMF<T>> = output.take(0)?;
        let item = result.iter().map(|tmf| {
            tmf.clone().item
        }).collect();
        Ok(item)    
    }

    pub async fn get_item<T : HasId + Serialize + Clone + DeserializeOwned>(&self, id : String, query_opts : QueryOptions) -> Result<Vec<T>,PlatypusError> {
        match Persistence::query_to_fields(query_opts) {
            Some(f) => self.get_tmf_item_fields(id, f).await,
            None => self.get_tmf_item(id).await
        }
    }

    pub async fn get_items<T : HasId + Serialize + Clone + DeserializeOwned>(&self, query_opts : QueryOptions) -> Result<Vec<T>,PlatypusError> {
        match Persistence::query_to_fields(query_opts.clone()) {
            Some(_f) => self.get_tmf_items_fields(query_opts.clone()).await,
            None => self.get_tmf_items(query_opts).await,
        }
    }

    pub async fn get_tmf_item<T : HasId + Serialize + Clone + DeserializeOwned>(&self,id : String) -> Result<Vec<T>,PlatypusError> {
        
        let query = format!("SELECT * FROM {}:{}",T::get_class(),id);
        let mut output = self.db.query(query).await?;
        let result : Vec<TMF<T>> = output.take(0)?;
        debug!("JSON: {}",serde_json::to_string(&result).unwrap());
        let item = result.iter().map(|tmf| {
            tmf.clone().item
        }).collect();
        Ok(item)
    }

    pub async fn get_tmf_item_fields<T : HasId + Serialize + Clone + DeserializeOwned>(&self, id : String, fields : Vec<String>) -> Result<Vec<T>,PlatypusError> {
        // Generate additional fields from vec
        let field_query = match fields.is_empty() {
            false => {
                let fields : Vec<String> = fields.into_iter().map(|f| {
                    // Standard payload has TMF payload under 'item' object thus need to prepend 'item' to each field.
                    format!("item.{f}")
                }).collect();
                format!(",{}",fields.join(","))
            },
            true => {
                String::new()
            }
        };

        let query = format!("SELECT item.id, item.href {} FROM {}:{}",field_query, T::get_class(),id);
        let mut output = self.db.query(query).with_stats().await?;
       
        //let result : Vec<TMF<T>> = output.take(0)?;
        let data = output.take(0);
        match data {
            Some(o) => {
                let (stats,result) = o;
                let _execution_time = stats.execution_time;

                let item_set: Vec<TMF<T>> = result?;
                let item = item_set.iter().map(|tmf| {
                    tmf.clone().item
                }).collect();
                Ok(item)
            },
            None => {
                Err(PlatypusError::from("No results found."))
            }
        }
    }

    pub async fn create_hub_item<T : Serialize + Clone + DeserializeOwned + 'static>(&self, item : T) -> Result<T,PlatypusError> {
       
        let result : Option<T> = self.db.create("hub")
            .content(item).await?;
        match result {
            Some(r) => Ok(r),
            None => Err(PlatypusError::from("Could not create object"))
        }
    }

    pub async fn delete_hub_item<T : Serialize + Clone + DeserializeOwned + 'static>(&self, id : String) -> Result<T,PlatypusError> {
        let result : Option<TMF<T>> = self.db.delete(("hub",id)).await?;
        match result {
            Some(r) => Ok(r.item),
            None => Err(PlatypusError::from("Could not delet hub entry")),
        }
    }

    /// Generate function to store into a db.
    pub async fn create_tmf_item<'a, T : HasId + Serialize + Clone + DeserializeOwned + 'static>(&self, mut item : T) -> Result<Vec<T>,PlatypusError> {
        // let class = T::get_class();
        // Should only generate a new id if one has not been supplied
        item.generate_id();
        let payload = Persistence::tmf_payload(item);
        // let tuple = (T::get_class(),item.get_id());
        let insert_option: Option<TMF<T>>  = self.db.create(T::get_class())
            .content(payload).await?;
        match insert_option {
            Some(o) => Ok(vec![o.item]),
            None => Err(PlatypusError::from("Could not create object"))
        }
    }

    pub async fn create_tmf_item_lastupdate<'a, T: HasId + HasLastUpdate + Serialize + Clone + DeserializeOwned + 'static>(&self, mut item : T) -> Result<Vec<T>,PlatypusError> {
        item.set_last_update(T::get_timestamp());
        self.create_tmf_item(item).await
    }

    pub async fn patch_tmf_item<T : HasId + Serialize + Clone + DeserializeOwned + 'static>(&self, id : String, mut patch : T) -> Result<Vec<T>,PlatypusError> {
        // We need to use id in the payload so need to ensure its set even if its not in the original payload
        patch.set_id(id);
        let payload = Persistence::tmf_payload(patch.clone());
        let result : Option<TMF<T>> = self.db.update((T::get_class(),patch.get_id()))
            .merge(payload).await?;
        match result {
            Some(r) => {
                Ok(vec![r.item])
            },
            None => Err(PlatypusError::from("Could not update object"))
        }
    }

    pub async fn patch_tmf_item_lastupdate<T : HasId + HasLastUpdate + Serialize + Clone + DeserializeOwned + 'static>(&self, id : String, mut patch : T) -> Result<Vec<T>,PlatypusError> {
        patch.set_last_update(T::get_timestamp());
        self.patch_tmf_item(id, patch).await
    }

    pub async fn delete_tmf_item<T>(&self, id : String) -> Result<T,PlatypusError>
    where
        T : HasId + Serialize + Clone + DeserializeOwned
    {
        //let resource = format!("({},{})",T::get_class(),id);
        // Need to generate a tuple, not just a string with brackets!
        let result : Option<TMF<T>> = self.db.clone().delete((T::get_class(),id)).await?;
        match result {
            Some(r) => Ok(r.item),
            None => Err(PlatypusError::from("Issue Deleting object")),
        }
    }

    #[cfg(feature = "events")]
    pub async fn store_tmf_event<T,U>(&self, event : Event<T, U>) -> Result<Event<T, U>,PlatypusError> 
    where 
        T : Serialize + Clone + DeserializeOwned + 'static,
        U : Serialize + DeserializeOwned + 'static
    {
        // Step1, store event in DB for processing.
        let domain = event.domain.clone();
        let result : Option<Event<T,U>> = self.db.create("event")
            .content(event)
            .await?;
        debug!("Event created, domain = {}",domain.clone().unwrap_or_default());
        match result {
            Some(e) => {
                // Trigger sending of events here for now
                let _send_result = self.send_tmf_events(domain).await;
                Ok(e)
            },
            None => Err(PlatypusError::from("Could not store event")),
        }
    }

    #[cfg(feature = "events")]
    pub async fn send_tmf_events(&self, domain : Option<String>) -> Result<u16,PlatypusError> {
        info!("Process events for domain: {}",domain.unwrap_or("No domain".to_string()));

        Err(PlatypusError::from("Not implemented"))
    }
}

mod tests {

    #[test]
    fn test_env_variable() {
        unsafe {
            std::env::set_var("PLATYPUS_DB_PATH", "/test/db/path");
        }

        let db_path = std::env::var("PLATYPUS_DB_PATH")
            .unwrap_or_else(|_| String::from("/home/rruckley/build/platypus/tmf.db"));

        assert_eq!(db_path, "/test/db/path");
    }
}

