use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ChangeController {
        identifier: Addr,
        new_controller: Addr,
    },
    SetAttribute {
        identifier: Addr,
        name: String,  // TODO: change to byte
        value: String, // TODO: change to byte
        validity: u64,
    },
    RevokeAttribute {
        identifier: Addr,
        name: String,  // TODO: change to byte
        value: String, // TODO: change to byte
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Controller { identifier: Addr },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ControllerResponse {
    pub controller: Addr,
}
