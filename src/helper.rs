use cosmwasm_std::{Addr, Attribute, Response};

use crate::error::ContractError;

pub fn only_controller(
    sender: &Addr,
    identifier: &Addr,
    loaded_controller: Option<Addr>,
) -> Result<(), ContractError> {
    match loaded_controller {
        Some(controller_address) => {
            if sender != &controller_address {
                return Err(ContractError::Unauthorized {});
            }
            Ok(())
        }
        None => {
            if sender != identifier {
                return Err(ContractError::Unauthorized {});
            }
            Ok(())
        }
    }
}

pub fn get_attribute_value(response: Response, target: &str) -> String {
    let attribute: Vec<Attribute> = response
        .attributes
        .into_iter()
        .filter(|attribute| attribute.key == *target.to_string())
        .collect();
    attribute[0].value.clone()
}
