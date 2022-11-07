use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::Map;

pub const CONTROLLERS: Map<&Addr, Addr> = Map::new("controller");
pub const CHANGED: Map<&Addr, u64> = Map::new("changed");
pub const NONCE: Map<&Addr, u64> = Map::new("nonce");
pub const ATTRIBUTES: Map<(&Addr, &String), Vec<String>> = Map::new("attribute");
pub const VALIDITIES: Map<(&Addr, &String, &String), Timestamp> = Map::new("validities");
