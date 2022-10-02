#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::helper::only_controller;
use crate::msg::{ControllerResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CHANGED, CONTROLLERS};

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
        ExecuteMsg::ChangeController {
            identifier,
            new_controller,
        } => try_change_controller(deps, env, info, identifier, new_controller),
        ExecuteMsg::SetAttribute {
            identifier,
            name,
            value,
            validity,
        } => try_set_attribute(deps, env, info, identifier, name, value, validity),
        ExecuteMsg::RevokeAttribute {
            identifier,
            name,
            value,
        } => try_revoke_attribute(deps, env, info, identifier, name, value),
    }
}

pub fn try_change_controller(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    identifier: Addr,
    new_controller: Addr,
) -> Result<Response, ContractError> {
    CONTROLLERS.update(
        deps.storage,
        &identifier,
        |loaded_controller: Option<Addr>| -> Result<_, ContractError> {
            only_controller(&info.sender, &identifier, loaded_controller)?;

            Ok(new_controller.clone())
        },
    )?;

    let loaded_changed = CHANGED.may_load(deps.storage, &identifier)?;
    let changed = loaded_changed.unwrap_or(0);

    let res = Response::new()
        .add_attribute("identifier", identifier.clone())
        .add_attribute("controller", new_controller)
        .add_attribute("previousChange", changed.to_string());

    CHANGED.update(
        deps.storage,
        &identifier,
        |_changed: Option<u64>| -> Result<_, ContractError> { Ok(env.block.height) },
    )?;

    Ok(res)
}

pub fn try_set_attribute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    identifier: Addr,
    name: String,
    value: String,
    validity: u64,
) -> Result<Response, ContractError> {
    // check controller
    let loaded_controller = CONTROLLERS.may_load(deps.storage, &identifier)?;
    only_controller(&info.sender, &identifier, loaded_controller)?;

    let loaded_changed = CHANGED.may_load(deps.storage, &identifier)?;
    let changed = loaded_changed.unwrap_or(0);

    let res = Response::new()
        .add_attribute("identifier", identifier.clone())
        .add_attribute("name", name)
        .add_attribute("value", value)
        .add_attribute("validTo", env.block.time.plus_seconds(validity).to_string())
        .add_attribute("previousChange", changed.to_string())
        .add_attribute("from", info.sender);

    CHANGED.update(
        deps.storage,
        &identifier,
        |_changed: Option<u64>| -> Result<_, ContractError> { Ok(env.block.height) },
    )?;

    Ok(res)
}

pub fn try_revoke_attribute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    identifier: Addr,
    name: String,
    value: String,
) -> Result<Response, ContractError> {
    // check controller
    let loaded_controller = CONTROLLERS.may_load(deps.storage, &identifier)?;
    only_controller(&info.sender, &identifier, loaded_controller)?;

    let loaded_changed = CHANGED.may_load(deps.storage, &identifier)?;
    let changed = loaded_changed.unwrap_or(0);

    let res = Response::new()
        .add_attribute("identifier", identifier.clone())
        .add_attribute("name", name)
        .add_attribute("value", value)
        .add_attribute("validTo", 0.to_string())
        .add_attribute("previousChange", changed.to_string())
        .add_attribute("from", info.sender);

    CHANGED.update(
        deps.storage,
        &identifier,
        |_changed: Option<u64>| -> Result<_, ContractError> { Ok(env.block.height) },
    )?;

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Controller { identifier } => to_binary(&query_controller(deps, identifier)?),
    }
}

fn query_controller(deps: Deps, identifier: Addr) -> StdResult<ControllerResponse> {
    let loaded_controller = CONTROLLERS.may_load(deps.storage, &identifier)?;
    match loaded_controller {
        Some(v) => Ok(ControllerResponse { controller: v }),
        None => Ok(ControllerResponse {
            controller: identifier,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::get_attribute_value;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

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
    fn controller() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identifier = String::from("identifier0001");

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Controller {
                identifier: Addr::unchecked(&identifier),
            },
        )
        .unwrap();
        let value: ControllerResponse = from_binary(&res).unwrap();
        assert_eq!(identifier, value.controller);
    }

    #[test]
    fn change_controller() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identifier1 = String::from("identifier0001");
        let controller1 = String::from("addr0001");

        // only the original identifier address can change the controller at the first time
        let auth_info = mock_info("identifier0001", &coins(2, "token"));

        let msg = ExecuteMsg::ChangeController {
            identifier: Addr::unchecked(&identifier1),
            new_controller: Addr::unchecked(&controller1),
        };

        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Controller {
                identifier: Addr::unchecked(&identifier1),
            },
        )
        .unwrap();
        let value: ControllerResponse = from_binary(&res).unwrap();
        assert_eq!(controller1, value.controller);

        // only the controller address can change the controller
        let auth_info = mock_info("addr0001", &coins(2, "token"));
        let controller2 = String::from("addr0002");

        let msg = ExecuteMsg::ChangeController {
            identifier: Addr::unchecked(&identifier1),
            new_controller: Addr::unchecked(&controller2),
        };

        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Controller {
                identifier: Addr::unchecked(&identifier1),
            },
        )
        .unwrap();
        let value: ControllerResponse = from_binary(&res).unwrap();
        assert_eq!(controller2, value.controller);
    }

    #[test]
    fn change_controller_by_attacker() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identifier1 = String::from("identifier0001");
        let controller1 = String::from("addr0001");

        // only the original identifier address can change the controller at the first time
        let auth_info = mock_info("attacker", &coins(2, "token"));

        let msg = ExecuteMsg::ChangeController {
            identifier: Addr::unchecked(&identifier1),
            new_controller: Addr::unchecked(&controller1),
        };

        let err = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Controller {
                identifier: Addr::unchecked(&identifier1),
            },
        )
        .unwrap();
        let value: ControllerResponse = from_binary(&res).unwrap();
        assert_eq!(identifier1, value.controller);

        // only the original identifier address can change the controller at the first time
        let auth_info = mock_info("identifier0001", &coins(2, "token"));

        let msg = ExecuteMsg::ChangeController {
            identifier: Addr::unchecked(&identifier1),
            new_controller: Addr::unchecked(&controller1),
        };

        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Controller {
                identifier: Addr::unchecked(&identifier1),
            },
        )
        .unwrap();
        let value: ControllerResponse = from_binary(&res).unwrap();
        assert_eq!(controller1, value.controller);

        let auth_info = mock_info("attacker", &coins(2, "token"));

        let msg = ExecuteMsg::ChangeController {
            identifier: Addr::unchecked(&identifier1),
            new_controller: Addr::unchecked(&identifier1),
        };

        let err = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Controller {
                identifier: Addr::unchecked(&identifier1),
            },
        )
        .unwrap();
        let value: ControllerResponse = from_binary(&res).unwrap();
        assert_eq!(controller1, value.controller);
    }

    #[test]
    fn set_attribute() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identifier1 = String::from("identifier0001");

        // only the original identifier address can change the controller at the first time
        let auth_info = mock_info("identifier0001", &coins(2, "token"));

        let msg = ExecuteMsg::SetAttribute {
            identifier: Addr::unchecked(&identifier1),
            name: String::from("identifier_name"),
            value: String::from("abc"),
            validity: 0,
        };

        let res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // check name attribute
        let name = get_attribute_value(res.clone(), "name");

        assert_eq!(name, "identifier_name");

        // check value attribute
        let value = get_attribute_value(res.clone(), "value");

        assert_eq!(value, "abc");
    }

    #[test]
    fn set_attribute_by_attacker() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identifier1 = String::from("identifier0001");

        // only the original identifier address can change the controller at the first time
        let auth_info = mock_info("attacker", &coins(2, "token"));

        let msg = ExecuteMsg::SetAttribute {
            identifier: Addr::unchecked(&identifier1),
            name: String::from("identifier_name"),
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

        let identifier1 = String::from("identifier0001");

        // only the original identifier address can change the controller at the first time
        let auth_info = mock_info("identifier0001", &coins(2, "token"));

        let msg = ExecuteMsg::SetAttribute {
            identifier: Addr::unchecked(&identifier1),
            name: String::from("identifier_name"),
            value: String::from("abc"),
            validity: 0,
        };

        let res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // check name attribute
        let name = get_attribute_value(res.clone(), "name");

        assert_eq!(name, "identifier_name");

        // check value attribute
        let value = get_attribute_value(res.clone(), "value");

        assert_eq!(value, "abc");

        //revoke_attribute test
        let msg = ExecuteMsg::RevokeAttribute {
            identifier: Addr::unchecked(&identifier1),
            name: String::from("identifier_name"),
            value: String::from("xyz"),
        };

        // only the original identifier address can change the controller at the first time
        let auth_info = mock_info("identifier0001", &coins(2, "token"));
        let res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // check name attribute
        let name = get_attribute_value(res.clone(), "name");

        assert_eq!(name, "identifier_name");

        // check value attribute
        let value = get_attribute_value(res.clone(), "value");

        assert_eq!(value, "xyz");

        // check validity attribute
        let validity = get_attribute_value(res.clone(), "validTo");

        assert_eq!(validity, "0");
    }

    #[test]
    fn revoke_attribute_by_attacker() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let identifier1 = String::from("identifier0001");

        // only the original identifier address can change the controller at the first time
        let auth_info = mock_info("identifier0001", &coins(2, "token"));

        let msg = ExecuteMsg::SetAttribute {
            identifier: Addr::unchecked(&identifier1),
            name: String::from("identifier_name"),
            value: String::from("abc"),
            validity: 0,
        };

        let res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // check name attribute
        let name = get_attribute_value(res.clone(), "name");

        assert_eq!(name, "identifier_name");

        // check value attribute
        let value = get_attribute_value(res.clone(), "value");

        assert_eq!(value, "abc");

        //revoke_attribute test
        let msg = ExecuteMsg::RevokeAttribute {
            identifier: Addr::unchecked(&identifier1),
            name: String::from("identifier_name"),
            value: String::from("xyz"),
        };

        // only the original identifier address can change the controller at the first time
        let auth_info = mock_info("attacker", &coins(2, "token"));
        let err = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }
}
