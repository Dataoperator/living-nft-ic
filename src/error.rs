use candid::CandidType;
use thiserror::Error;

#[derive(Error, Debug, CandidType)]
pub enum NftError {
    #[error("NFT not found")]
    NotFound,
    
    #[error("Not authorized")]
    NotAuthorized,
    
    #[error("Interaction failed: {0}")]
    InteractionFailed(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}
