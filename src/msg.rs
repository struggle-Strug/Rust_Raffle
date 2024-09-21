use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cosmwasm_std::Binary;
use cosmwasm_std::Coin;

use crate::state::GameState;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub authkey: String,
    pub owner: Addr
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReceiveNft {
        sender:  String,
        token_id: String,
        msg: Binary
    },
    StartRaffle {
        ticket_price: u64,
        total_ticket_count: u64.
        nft_contract_addr: Addr,
        nft_token_id: String,
        collection_wallet: Addr, // Collection wallet address to send tokens after the game finished
        end_time: u64,
    },
    EnterRaffle {
        game_id: u64
    },
    TransferTokensToCollectionWallet {
        amount: u128,
        denom: String,
        collection_wallet_address: String,
    },
    SelectWinnerAndTransferNFTtoWinner { game_id: u64 },
}