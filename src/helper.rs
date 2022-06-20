use cosmwasm_std::{Addr, Attribute, Response};

use crate::error::ContractError;

pub fn only_identity_owner(
    sender: &Addr,
    identity: &Addr,
    loaded_owner: Option<Addr>,
) -> Result<(), ContractError> {
    match loaded_owner {
        Some(owner_address) => {
            if sender != &owner_address {
                return Err(ContractError::Unauthorized {});
            }
            Ok(())
        }
        None => {
            if sender != identity {
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
