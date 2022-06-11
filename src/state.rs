use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

pub const OWNERS: Map<&Addr, Addr> = Map::new("owner");
pub const CHANGED: Map<&Addr, u64> = Map::new("changed");
pub const NONCE: Map<&Addr, u64> = Map::new("nonce");
