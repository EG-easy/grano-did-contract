use cosmwasm_std::{Attribute, Response};

pub fn get_attribute_value(response: Response, target: &str) -> String {
    let attribute: Vec<Attribute> = response
        .attributes
        .into_iter()
        .filter(|attribute| attribute.key == *target.to_string())
        .collect();
    attribute[0].value.clone()
}
