use cosmwasm_std::{Addr, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Controller {
        identifier: Addr,
    },
    Attribute {
        identifier: Addr,
        name: String,
    },
    ValidTo {
        identifier: Addr,
        name: String,
        value: String,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ControllerResponse {
    pub controller: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AttributeResponse {
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ValidToResponse {
    pub valid_to: Timestamp,
}
