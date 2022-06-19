#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg};
use crate::state::{CHANGED, OWNERS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:did-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ChangeOwner {
            identity,
            new_owner,
        } => try_change_owner(deps, env, info, identity, new_owner),
        ExecuteMsg::SetAttribute {
            identity,
            name,
            value,
            validity,
        } => try_set_attribute(deps, env, info, identity, name, value, validity),
        ExecuteMsg::RevokeAttribute {
            identity,
            name,
            value,
        } => try_revoke_attribute(deps, env, info, identity, name, value),
    }
}

pub fn try_change_owner(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    identity: Addr,
    new_owner: Addr,
) -> Result<Response, ContractError> {
    OWNERS.update(
        deps.storage,
        &identity,
        |owner: Option<Addr>| -> Result<_, ContractError> {
            match owner {
                Some(owner_address) => {
                    if info.sender != owner_address {
                        return Err(ContractError::Unauthorized {});
                    }
                }
                None => {
                    if info.sender != identity {
                        return Err(ContractError::Unauthorized {});
                    }
                }
            }
            Ok(new_owner.clone())
        },
    )?;

    let loaded_changed = CHANGED.may_load(deps.storage, &identity)?;
    let changed = loaded_changed.unwrap_or(0);

    let res = Response::new()
        .add_attribute("identity", identity.clone())
        .add_attribute("owner", new_owner)
        .add_attribute("previousChange", changed.to_string());

    CHANGED.update(
        deps.storage,
        &identity,
        |_changed: Option<u64>| -> Result<_, ContractError> { Ok(env.block.height) },
    )?;

    Ok(res)
}

pub fn try_set_attribute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    identity: Addr,
    name: String,
    value: String,
    validity: u64,
) -> Result<Response, ContractError> {
    // check owner
    let loaded_owner = OWNERS.may_load(deps.storage, &identity)?;
    match loaded_owner {
        Some(owner_address) => {
            if info.sender != owner_address {
                return Err(ContractError::Unauthorized {});
            }
        }
        None => {
            if info.sender != identity {
                return Err(ContractError::Unauthorized {});
            }
        }
    }

    let loaded_changed = CHANGED.may_load(deps.storage, &identity)?;
    let changed = loaded_changed.unwrap_or(0);

    let res = Response::new()
        .add_attribute("identity", identity.clone())
        .add_attribute("name", name)
        .add_attribute("value", value)
        .add_attribute("validTo", env.block.time.plus_seconds(validity).to_string())
        .add_attribute("previousChange", changed.to_string())
        .add_attribute("from", info.sender);

    CHANGED.update(
        deps.storage,
        &identity,
        |_changed: Option<u64>| -> Result<_, ContractError> { Ok(env.block.height) },
    )?;

    Ok(res)
}

pub fn try_revoke_attribute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    identity: Addr,
    name: String,
    value: String,
) -> Result<Response, ContractError> {
    // check owner
    let loaded_owner = OWNERS.may_load(deps.storage, &identity)?;
    match loaded_owner {
        Some(owner_address) => {
            if info.sender != owner_address {
                return Err(ContractError::Unauthorized {});
            }
        }
        None => {
            if info.sender != identity {
                return Err(ContractError::Unauthorized {});
            }
        }
    }
    let loaded_changed = CHANGED.may_load(deps.storage, &identity)?;
    let changed = loaded_changed.unwrap_or(0);

    let res = Response::new()
        .add_attribute("identity", identity.clone())
        .add_attribute("name", name)
        .add_attribute("value", value)
        .add_attribute("validTo", 0.to_string())
        .add_attribute("previousChange", changed.to_string())
        .add_attribute("from", info.sender);

    CHANGED.update(
        deps.storage,
        &identity,
        |_changed: Option<u64>| -> Result<_, ContractError> { Ok(env.block.height) },
    )?;

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::IdentityOwner { identity } => to_binary(&query_owner(deps, identity)?),
    }
}

fn query_owner(deps: Deps, identity: Addr) -> StdResult<OwnerResponse> {
    let loaded_owner = OWNERS.may_load(deps.storage, &identity)?;
    match loaded_owner {
        Some(v) => Ok(OwnerResponse { owner: v }),
        None => Ok(OwnerResponse { owner: identity }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Attribute};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn identity_owner() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identity1 = String::from("identity0001");

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::IdentityOwner {
                identity: Addr::unchecked(&identity1),
            },
        )
        .unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(identity1, value.owner);
    }

    #[test]
    fn change_owner() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identity1 = String::from("identity0001");
        let owner1 = String::from("addr0001");

        // only the original identity address can change the owner at the first time
        let auth_info = mock_info("identity0001", &coins(2, "token"));

        let msg = ExecuteMsg::ChangeOwner {
            identity: Addr::unchecked(&identity1),
            new_owner: Addr::unchecked(&owner1),
        };

        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::IdentityOwner {
                identity: Addr::unchecked(&identity1),
            },
        )
        .unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(owner1, value.owner);

        // only the owner address can change the owner
        let auth_info = mock_info("addr0001", &coins(2, "token"));
        let owner2 = String::from("addr0002");

        let msg = ExecuteMsg::ChangeOwner {
            identity: Addr::unchecked(&identity1),
            new_owner: Addr::unchecked(&owner2),
        };

        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::IdentityOwner {
                identity: Addr::unchecked(&identity1),
            },
        )
        .unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(owner2, value.owner);
    }

    #[test]
    fn change_owner_by_attacker() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identity1 = String::from("identity0001");
        let owner1 = String::from("addr0001");

        // only the original identity address can change the owner at the first time
        let auth_info = mock_info("attacker", &coins(2, "token"));

        let msg = ExecuteMsg::ChangeOwner {
            identity: Addr::unchecked(&identity1),
            new_owner: Addr::unchecked(&owner1),
        };

        let err = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::IdentityOwner {
                identity: Addr::unchecked(&identity1),
            },
        )
        .unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(identity1, value.owner);

        // only the original identity address can change the owner at the first time
        let auth_info = mock_info("identity0001", &coins(2, "token"));

        let msg = ExecuteMsg::ChangeOwner {
            identity: Addr::unchecked(&identity1),
            new_owner: Addr::unchecked(&owner1),
        };

        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::IdentityOwner {
                identity: Addr::unchecked(&identity1),
            },
        )
        .unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(owner1, value.owner);

        let auth_info = mock_info("attacker", &coins(2, "token"));

        let msg = ExecuteMsg::ChangeOwner {
            identity: Addr::unchecked(&identity1),
            new_owner: Addr::unchecked(&identity1),
        };

        let err = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::IdentityOwner {
                identity: Addr::unchecked(&identity1),
            },
        )
        .unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(owner1, value.owner);
    }

    #[test]
    fn set_attribute() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identity1 = String::from("identity0001");

        // only the original identity address can change the owner at the first time
        let auth_info = mock_info("identity0001", &coins(2, "token"));

        let msg = ExecuteMsg::SetAttribute {
            identity: Addr::unchecked(&identity1),
            name: String::from("identity_name"),
            value: String::from("abc"),
            validity: 0,
        };

        let res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // check name attribute
        let name_attribute: Vec<Attribute> = res
            .clone()
            .attributes
            .into_iter()
            .filter(|attribute| attribute.key == "name")
            .collect();

        assert_eq!(name_attribute[0].value, "identity_name");

        // check value attribute
        let value_attribute: Vec<Attribute> = res
            .attributes
            .into_iter()
            .filter(|attribute| attribute.key == "value")
            .collect();

        assert_eq!(value_attribute[0].value, "abc");
    }

    #[test]
    fn set_attribute_by_attacker() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identity1 = String::from("identity0001");

        // only the original identity address can change the owner at the first time
        let auth_info = mock_info("attacker", &coins(2, "token"));

        let msg = ExecuteMsg::SetAttribute {
            identity: Addr::unchecked(&identity1),
            name: String::from("identity_name"),
            value: String::from("abc"),
            validity: 0,
        };

        let err = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }

    #[test]
    fn revoke_attribute() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identity1 = String::from("identity0001");

        // only the original identity address can change the owner at the first time
        let auth_info = mock_info("identity0001", &coins(2, "token"));

        let msg = ExecuteMsg::SetAttribute {
            identity: Addr::unchecked(&identity1),
            name: String::from("identity_name"),
            value: String::from("abc"),
            validity: 0,
        };

        let res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // check name attribute
        let name_attribute: Vec<Attribute> = res
            .clone()
            .attributes
            .into_iter()
            .filter(|attribute| attribute.key == "name")
            .collect();

        assert_eq!(name_attribute[0].value, "identity_name");

        // check value attribute
        let value_attribute: Vec<Attribute> = res
            .attributes
            .into_iter()
            .filter(|attribute| attribute.key == "value")
            .collect();

        assert_eq!(value_attribute[0].value, "abc");

        //revoke_attribute test
        let msg = ExecuteMsg::RevokeAttribute {
            identity: Addr::unchecked(&identity1),
            name: String::from("identity_name"),
            value: String::from("xyz"),
        };

        // only the original identity address can change the owner at the first time
        let auth_info = mock_info("identity0001", &coins(2, "token"));
        let res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        println!("res: {:?} ", res);

        // check name attribute
        let name_attribute: Vec<Attribute> = res
            .clone()
            .attributes
            .into_iter()
            .filter(|attribute| attribute.key == "name")
            .collect();

        assert_eq!(name_attribute[0].value, "identity_name");

        // check value attribute
        let value_attribute: Vec<Attribute> = res
            .clone()
            .attributes
            .into_iter()
            .filter(|attribute| attribute.key == "value")
            .collect();

        assert_eq!(value_attribute[0].value, "xyz");

        // check validity attribute
        let value_attribute: Vec<Attribute> = res
            .attributes
            .into_iter()
            .filter(|attribute| attribute.key == "validTo")
            .collect();

        assert_eq!(value_attribute[0].value, "0");
    }

    #[test]
    fn revoke_attribute_by_attacker() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identity1 = String::from("identity0001");

        // only the original identity address can change the owner at the first time
        let auth_info = mock_info("identity0001", &coins(2, "token"));

        let msg = ExecuteMsg::SetAttribute {
            identity: Addr::unchecked(&identity1),
            name: String::from("identity_name"),
            value: String::from("abc"),
            validity: 0,
        };

        let res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // check name attribute
        let name_attribute: Vec<Attribute> = res
            .clone()
            .attributes
            .into_iter()
            .filter(|attribute| attribute.key == "name")
            .collect();

        assert_eq!(name_attribute[0].value, "identity_name");

        // check value attribute
        let value_attribute: Vec<Attribute> = res
            .attributes
            .into_iter()
            .filter(|attribute| attribute.key == "value")
            .collect();

        assert_eq!(value_attribute[0].value, "abc");

        //revoke_attribute test
        let msg = ExecuteMsg::RevokeAttribute {
            identity: Addr::unchecked(&identity1),
            name: String::from("identity_name"),
            value: String::from("xyz"),
        };

        // only the original identity address can change the owner at the first time
        let auth_info = mock_info("attacker", &coins(2, "token"));
        let err = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }
}
