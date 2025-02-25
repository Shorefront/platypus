//! Hub module for end-point registration

use actix_web::{post,delete,web, HttpResponse, Responder};
use super::error::PlatypusError;
use crate::common::persist::Persistence;
use std::sync::Mutex;
use log::error;
use serde::{Deserialize,Serialize};
use tmflib::Uri;
use crate::common::persist::TMF;


#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct NotificationEndpoint {
    id: Option<String>,
    domain : String,
    filter : Option<String>,
    callback: Uri,
    query: Option<String>,
}

#[derive(Clone, Debug)]
pub struct HubManagement {
    persist : Option<Persistence>,
}

impl HubManagement {
    pub fn new(persist : Option<Persistence>) -> Self {
        HubManagement {
            persist,
        }
    }
    pub fn persist(&mut self, persist : Persistence) {
        self.persist = Some(persist);
    }

    pub async fn register_hub(&mut self, hub : NotificationEndpoint) -> Result<NotificationEndpoint,PlatypusError> {
        let payload = TMF {
            id : ("hub",hub.id.clone().unwrap()).into(),
            item : hub.clone(),
        };
        self
            .persist
            .as_mut()
            .unwrap()
            .create_hub_item(payload).await
            .map(|r| r.item)
    }   

    pub async fn unregister_hub(&mut self, hub_id : String) -> Result<NotificationEndpoint,PlatypusError> {
        self.persist.as_mut().unwrap().delete_hub_item(hub_id).await
    }
}

pub fn render_register_hub(output : Result<NotificationEndpoint,PlatypusError>) -> HttpResponse {
    match output {
        Ok(b) => {
            HttpResponse::Created()
                .append_header(("Location",format!("/tmf-api/hub/{}",b.id.clone().unwrap())))
                .append_header(("Content-Type","application/json"))
                .json(b.clone())
        }
        Err(e) => {
            error!("Could not create: {e}");
            HttpResponse::Conflict().json(e)
        },     
    }
}

pub fn render_delete_hub<T : Serialize>(output : Result<T,PlatypusError>) -> HttpResponse {
    match output {
        Ok(_b) => HttpResponse::NoContent().finish(),
        Err(e) => {
            error!("Could not delete: {e}");
            HttpResponse::NotFound().json(e)
        },     
    }     
}

#[post("/tmf-api/hub")]
pub async fn hub_handle_post(
    raw: web::Bytes,
    hub: web::Data<Mutex<HubManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let mut hub = hub.lock().unwrap();
    hub.persist(persist.lock().unwrap().clone());

    let json = String::from_utf8(raw.to_vec()).unwrap();

    let mut end_point : NotificationEndpoint = match serde_json::from_str(&json) {
        Ok(e) => e,
        Err(e) => {
            return HttpResponse::BadRequest().json(e.to_string());
        }
    };
    if end_point.id.is_none() {
        let id = uuid::Uuid::new_v4();
        let (short,_) = tmflib::gen_code(format!("{}:{}","H-",end_point.domain), id.to_string(), None, Some(String::from("HUB")), None);
        end_point.id = Some(short);
    }
    let response = hub.register_hub(end_point).await;
    render_register_hub(response)
}

#[delete("/tmf-api/hub/{hub_id}")]
pub async fn hub_handle_delete(
    path : web::Path<String>,
    hub: web::Data<Mutex<HubManagement>>,
    persist: web::Data<Mutex<Persistence>>,
) -> impl Responder {
    let id = path.into_inner();
    let mut hub = hub.lock().unwrap();
    hub.persist(persist.lock().unwrap().clone());

    let result = hub.unregister_hub(id).await;

    render_delete_hub(result)
}

pub fn config_hub(cfg: &mut web::ServiceConfig) {
    let hub_mgt = HubManagement::new(None);
    cfg
        .service(hub_handle_post)
        .service(hub_handle_delete)
        .app_data(web::Data::new(Mutex::new(hub_mgt.clone())));
}