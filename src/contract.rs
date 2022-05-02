#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{RankResponse, InstantiateMsg, QueryMsg, NameResponse};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:clicker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // We're storing stuff in a variable called "state" of type "State"
    let state = State {
      rank: msg.rank,
      owner: info.sender.clone(),
      name: msg.name,
    };
  
    // We're setting the contract version using a helper function we imported
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // We're storing state in a special variable called "STATE"
    STATE.save(deps.storage, &state)?;
  
    // Sending a response back to the caller
    Ok(Response::new()
      .add_attribute("method", "instantiate")
      .add_attribute("owner", info.sender)
      .add_attribute("rank", msg.rank.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
      QueryMsg::GetRank {} => to_binary(&query_rank(deps)?),
      QueryMsg::GetName {} => to_binary(&query_name(deps)?),
  }
}

fn query_rank(deps: Deps) -> StdResult<RankResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(RankResponse { rank: state.rank })
}

fn query_name(deps: Deps) -> StdResult<NameResponse> {
  let state = STATE.load(deps.storage)?;
  Ok(NameResponse { name: state.name})
}