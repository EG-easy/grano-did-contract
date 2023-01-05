use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::Map;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Attribute {
    pub values: Vec<String>,
}

pub const CONTROLLERS: Map<&Addr, Addr> = Map::new("controller");
pub const CHANGED: Map<&Addr, u64> = Map::new("changed");
pub const NONCE: Map<&Addr, u64> = Map::new("nonce");

pub const ATTRIBUTES: Map<(&Addr, &str), Attribute> = Map::new("attribute");
pub const VALIDITIES: Map<(&Addr, &str, &str), Timestamp> = Map::new("validities");
