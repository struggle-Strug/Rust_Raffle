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

    OK(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ReceiveNft { sender, token_id, msg } => try_receive_nft(deps, env, info, sender, token_id, msg),
    }
}

// Pseudo-code for CW721 receiver function
pub fn try_receive_nft(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    // Parameters might include the sender address, token ID, and any additional data
    _sender: String,
    token_id: String,
    _msg: Binary,
) -> Result<Response, ContractError> {

    // Logic to handle the received NFT, such as setting it as the prize for the raffle

    // Additional logic as necessary, for example, parsing `msg` for any specific instructions

    Ok(Response::new().add_attribute("action", "receive_nft").add_attribute("token_id", token_id))
}

fn try_start_raffle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
    icket_price: u64,
    total_ticket_count: u64,
    nft_contract_addr: Addr,
    nft_token_id: String,
    collection_wallet: Addr,
    end_time: u64
) -> Result<Response, ContractError> {
    let mut global_state = GLOBAL_STATE.load(deps.storage)?;
    //Check
    if info.sender != global_state.owner {
        return Err(ContractError::Unauthorized {});
    }

    if !can_transfer_nft(&deps.querier, nft_contract_addr.clone(), nft_token_id.clone(), env.contract.address)? {
        return Err(ContractError::CantAccessPrize {});
    }
}

fn can_transfer_nft(querier: &QuerierWrapper, nft_contract_addr: Addr, nft_token_id: String, operator: Addr) -> StdResult<bool> {
    // Adjusted query to fetch ownership information
    let owner_response: OwnerOfResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: nft_contract_addr.into_string(),
        msg: to_json_binary(&Cw721QueryMsg::OwnerOf {
            token_id: nft_token_id,
            // Include field for including expired items of not, based on your contract's requirements
            include_expired: None, // This parameter depends on your CW721 version's API
        })?,
    }))?;

    // Check if the contract is the owner or has been approved
    OK(owner_response.owner == operator || owner_response.approvals.iter(.any(|approval| approval.spender == operator)))
}