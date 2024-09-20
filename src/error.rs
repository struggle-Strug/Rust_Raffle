use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Can not access prize NFT")]
    CantAccessPrize {},
    #[error("Raffle already ended")]
    RaffleEnded {},
    #[error("Raffle Time Over")]
    RaffleTimeOver {},
    #[error("Raffle Time Over")]
    RaffleTimeOver {},
    #[error("All raffle tickets was sold.")]
    RaffleSoldOut {},
    #[error("Incorrect Funds")]
    IncorrectFunds {},
}
