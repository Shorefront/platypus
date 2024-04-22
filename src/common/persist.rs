//! Persistence Module
//! 
use surrealdb::engine::local::Db;
use surrealdb::engine::local::SpeeDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use log::debug;

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tmflib::common::event::Event;

use crate::QueryOptions;
use super::error::PlatypusError;
use tmflib::HasId;

use std::env;

/// Generic TMF struct for DB
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TMF<T : HasId> {
    id : Option<Thing>,
    pub item : T,
}

#[derive(Clone,Debug)]
pub struct Persistence {
    pub db : Surreal<Db>,
}

impl Persistence {
    pub async fn new() -> Persistence {
        let db_path = env::var("PLATYPUS_DB_PATH")
            .unwrap_or_else(|_| String::from("/home/rruckley/build/platypus/tmf.db"));
        let db = Surreal::new::<SpeeDb>(&db_path)
            .await
            .expect("Could not open DB connection");
        db.use_ns("tmflib").use_db("composable").await.expect("Could not set DB NS");
        Persistence { db }
    }
}

impl Persistence {
    /// Geneate a TMF payload for storing in the database
    fn tmf_payload<'a, T : HasId + Serialize + Clone + Deserialize<'a>>(item : T) -> TMF<T> {
        TMF {
            id : Some(Thing {
                tb : T::get_class(),
                id : item.get_id().into(),
            }),
            item,
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
                if f == "none" || f == "" {
                    return Some(output);
                };
                f.split(',').into_iter().for_each(|f| {
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
            None => format!(""),
        };

        let offset = match query_opts.offset {
            Some(o) => format!("START AT {}",o),
            None => format!(""),
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
            None => format!(""),
        };

        let offset = match query_opts.offset {
            Some(o) => format!("START AT {}",o),
            None => format!(""),
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

    pub async fn get_items_filter<T : HasId + Serialize + Clone + DeserializeOwned>(&self, filter : String, _query_opts : QueryOptions) -> Result<Vec<T>,PlatypusError> {
        let query = format!("SELECT * FROM {} WHERE {}",T::get_class(),filter);
        let mut output = self.db.query(query).await?;
        let result : Vec<TMF<T>> = output.take(0)?;
        let item = result.iter().map(|tmf| {
            tmf.clone().item
        }).collect();
        Ok(item)         
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
        let mut output = self.db.query(query).await?;
        let result : Vec<TMF<T>> = output.take(0)?;
        let item = result.iter().map(|tmf| {
            tmf.clone().item
        }).collect();
        Ok(item)    
    }

    /// Generate function to store into a db.
    pub async fn create_tmf_item<'a, T : HasId + Serialize + Clone + DeserializeOwned>(&mut self, mut item : T) -> Result<Vec<T>,PlatypusError> {
        let class = T::get_class();
        // Should only generate a new id if one has not been supplied
        item.generate_id();
        let payload = Persistence::tmf_payload(item);
        let insert_records : Vec<TMF<T>> = self.db.create(class).content(payload).await?;
        let output = insert_records.into_iter().map(|tmf| {
            tmf.item
        }).collect();
        Ok(output)
    }

    pub async fn patch_tmf_item<T : HasId + Serialize + Clone + DeserializeOwned>(&self, id : String, patch : String) -> Result<Vec<T>,PlatypusError> {
        let resource = format!("({},{})",T::get_class(),id);
        let result : Result<Vec<TMF<T>>,_> = self.db.update(resource)
            .merge(patch).await;
        match result {
            Ok(r) => {
                Ok(r.into_iter().map(|tmf| {
                    tmf.item
                }).collect())
            },
            Err(e) => Err(PlatypusError::from(e))
        }
    }

    pub async fn delete_tmf_item<T : HasId + Serialize + Clone + DeserializeOwned>(&self, id : String) -> Result<bool,PlatypusError> {
        //let resource = format!("({},{})",T::get_class(),id);
        // Need to generate a tuple, not just a string with brackets!
        let result : Option<TMF<T>> = self.db.clone().delete((T::get_class(),id)).await?;
        match result {
            Some(_r) => Ok(true),
            None => Err(PlatypusError::from("Issue Deleting object")),
        }
    }

    pub async fn send_tmf_event<T : Serialize + Clone + DeserializeOwned,U>(&self, event : Event<T, U>) -> Result<bool,PlatypusError> {
        // First step, determine the domain of the event to filter hub entries
        if event.description.is_some() {
            debug!("Trying to send event: {}",event.description.unwrap());
        };
        Err(PlatypusError::from("Not implemented"))
    }
}

mod tests {

    #[test]
    fn test_env_variable() {
        std::env::set_var("PLATYPUS_DB_PATH", "/test/db/path");

        let db_path = std::env::var("PLATYPUS_DB_PATH")
            .unwrap_or_else(|_| String::from("/home/rruckley/build/platypus/tmf.db"));

        assert_eq!(db_path, "/test/db/path");
    }
}

