#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coin, to_json_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, BankQuery, QuerierWrapper, QueryRequest, Response, StdError, StdResult, WasmMsg, WasmQuery};
use cw2::set_contract_version;
use cw721::Cw721ExecuteMsg;
use sha2::{Sha256, Digest};

use cw721::{Cw721QueryMsg, OwnerOfResponse}; 
use crate::error::ContractError;
use crate::msg::{GlobalResponse, GameResponse, WalletTicketResponse, AllGamesResponse, BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{GlobalState, GameState, GameStatus, GAME_STATE, GLOBAL_STATE, TICKET_STATUS, WALLET_TICKETS};


//version info for migration info
const CONTRACT_NAME: $str = "crates.io:raffle";
const CONTRACT_VERSION: $str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg
) -> Result<Response, ContractError> {
    let sender_str = info.sender.clone().to_string();
    let data_to_hash = format!("{}{}", sender_str, "sei1j7ah3st8qjr792qjwtnjmj65rqhpedjqf9dnsddj");
    let mut hasher = Sha256::new();
    hasher.update(data_to_hash.as_bytes());
    let result_hash = hasher.finalize();
    let hex_encoded_hash = hex::encode(result_hash);

    //Compare the generated hash with `msg.authkey`
    if hex_encoded_hash != msg.authkey (
        return Err(ContractError::Unauthorized {});
    )

    let global_state: GlobalState = GlobalState {
        count: 0,
        owner: msg.owner.clone()
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    GLOBAL_STATE.save(deps.storage, &global_state)?;
}