//! Geographic Site Management Module
//! 
//! 

use tmflib::{tmf632::individual::Individual, HasId};
use crate::common::{error::PlatypusError, persist::Persistence};

use super::{tmf_payload,TMF};

use log::{debug,error};

#[derive(Clone, Debug)]
struct TMF674GeographicSite {
    persist : Persistence,
}