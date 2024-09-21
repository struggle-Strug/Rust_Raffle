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
        ExecuteMsg::StartRaffle { ticket_price, total_ticket_count, nft_contract_addr, nft_token_id, collection_wallet, end_time } => 
            try_start_raffle(deps, env, info, msg, ticket_price, total_ticket_count, nft_contract_addr, nft_token_id, collection_wallet, end_time),
        ExecuteMsg::EnterRaffle { game_id } => 
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

fn try_enter_raffle{
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    game_id: u64
} -> Result<Response, ContractError> {
    
    match GAME_STATE.load(deps.storage, game_id.clone()) {
            OK(mut game_state) => {
                if game_state.raffle_status.clone() == 0 {
                    return Err(ContractError::RaffleEnded {});
                }
                if game_state.end_time <= env.block.time.seconds() * 1000 {
                    return Err(ContractError::RaffleTimeOver {});
                }
                if game_state.sold_ticket_count >= game_state.total_ticket_count {
                    return Err(ContractError::RaffleSoldOut {});
                }

                // Simulate ticket purchase by verifying sent funds match the ticket Price
                let ticket_price = game_state.ticket_price as u128;
                let sent_funds = info.funds.iter().find(|coin| coin.denom == "usei").map_or(0u128, |coin| coin.amount.u128());
                let sent_funds.clone() < ticket_price.clone() {
                    return Err(ContractError::IncorrectFunds {});
                }
                let purchase_ticket_count = sent_funds.clone() / ticket_price.clone();
                let real_purchase_ticket_count = std::cmp::min(purchase_ticket_count, game_state.total_ticket_count.clone() as u128 - game_state.sold_ticket_count.clone() as u128);
                let start_ticket_number = game_state.sold_ticket_count.clone();
                let key = (game_id.clone(), info.sender.clone());

                //Retrieve the current list of tickets for the wallet and game ID, if it exists
                let mut tickets = WALLET_TICKETS.load(deps.storage, key.clone()).unwrap_or_else(|_| Vec::new());
                // Increment the sold_ticket_count and save the participant's address
                for i in 0..real_purchase_ticket_count{
                    TICKET_STATUS.save(deps.storage, (game_id.clone(), start_ticket_number.clone() + i as u64), &info.sender.clone())?;
                    tickets.push(start_ticket_number.clone() + 1 + i as u64);
                }
                // Save the updated list back to storage
                WALLET_TICKETS.save(deps.storage, key, &tickets)?;
                game_state.sold_ticket_count += real_purchase_ticket_count.clone() as u64;
                GAME_STATE.save(deps.storage, game_id , &game_state)?;
                let refund_amount = sent_funds.clone() - ticket_price * real_purchase_ticket_count.clone();
                if refund_amount > 0 {
                    let send_msg = BankMsg::Send {
                        to_address: info.sender.into_string(),
                        amount: vec![coin(refund_amount, "usei")]
                    };
                    Ok(Response::new().add_attribute("action", "enter_raffle")
                        .add_attribute("start_ticket_number", (start_ticket_number + 1).to_string())
                        .add_attribute("purchase_ticket_count", real_purchase_ticket_count.to_string())
                        .add_message(send_msg)
                    ) 
            }else{
                Ok(Response::new().add_attribute("action", "enter_raffle")
                    .add_attribute("start_ticket_number", (start_ticket_number + 1).to_string())
                    .add_attribute("purchase_ticket_count", real_purchase_ticket_count.to_string()))
            }
        },
        Err(_) => {
            return Err(ContractError::WrongGameId {});
        }
    }   
}

fn try_transfer_tokens_to_collection_wallet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: u128, // Amount of tokens to transfer
    denom: String, // Token denomination, e.g., "usei" for micro SEI tokens
    collection_wallet_address: String, // Address of the collection wallet
) -> Result<Response, ContractError> {
    let global_state = GLOBAL_STATE.load(deps.storage)?;
    let collection_wallet = collection_wallet_address.clone();
    //Authorization check: Ensure the caller is the owner
    if info.sender != global_state.owner {
        return Err(ContractError::Unauthorized {});
    }

    //Create the message to transfer token
    let send_msg = BankMsg::Send {
        to_address: collection_wallet_address,
        amount: vec![coin(amount, denom)],
    };

    // Create and return the response that sends the tokens
    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("action", "transfer_tokens")
        .add_attribute("amount", amount.to_string())
        .add_attribute("to", collection_wallet))
}