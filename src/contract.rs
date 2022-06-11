#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg};
use crate::state::{State, CHANGED, OWNERS, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:did-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::ChangeOwner {
            identity,
            new_owner,
        } => try_change_owner(deps, info, identity, new_owner),
        ExecuteMsg::SetAttribute {
            identity,
            name,
            value,
            validity,
        } => try_set_attribute(deps, info, identity, name, value, validity),
        ExecuteMsg::RevokeAttribute {
            identity,
            name,
            value,
        } => try_revoke_attribute(deps, info, identity, name, value),
    }
}

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

pub fn try_change_owner(
    deps: DepsMut,
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
            Ok(new_owner)
        },
    )?;

    let loaded_changed = CHANGED.may_load(deps.storage, &identity)?;
    let changed = loaded_changed.unwrap_or(0);

    let res = Response::new()
        .add_attribute("identity", identity)
        .add_attribute("changed", changed.to_string());
    Ok(res)
}

pub fn try_set_attribute(
    deps: DepsMut,
    info: MessageInfo,
    identity: Addr,
    name: String,
    value: String,
    validity: i32,
) -> Result<Response, ContractError> {
    CHANGED.update(
        deps.storage,
        &identity,
        |changed: Option<i32>| -> Result<_, ContractError> {
            Ok(changed.unwrap_or_default() + validity)
        },
    )?;

    let res = Response::new()
        .add_attribute("identity", identity)
        .add_attribute("name", name)
        .add_attribute("value", value)
        .add_attribute("from", info.sender);
    // TODO: update attribute
    Ok(res)
}

pub fn try_revoke_attribute(
    deps: DepsMut,
    info: MessageInfo,
    identity: Addr,
    name: String,
    value: String,
) -> Result<Response, ContractError> {
    CHANGED.update(
        deps.storage,
        &identity,
        |changed: Option<i32>| -> Result<_, ContractError> { Ok(changed.unwrap_or_default() + 1) },
    )?;
    let res = Response::new()
        .add_attribute("identity", identity)
        .add_attribute("name", name)
        .add_attribute("value", value)
        .add_attribute("from", info.sender);
    // TODO: update attribute
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::IdentityOwner { identity } => to_binary(&query_owner(deps, identity)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
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
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }

    #[test]
    fn identity_owner() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg { count: 17 };
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
        let msg = InstantiateMsg { count: 17 };
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
        let msg = InstantiateMsg { count: 17 };
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
        let msg = InstantiateMsg { count: 17 };
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
    }
}
