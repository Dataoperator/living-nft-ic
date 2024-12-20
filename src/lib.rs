use candid::{CandidType, Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

mod personality;
mod memory;
mod types;
mod error;

use personality::Personality;
use memory::Memory;
use types::*;
use error::NftError;

// Type aliases
type Memory = VirtualMemory<DefaultMemoryImpl>;
type Result<T> = std::result::Result<T, NftError>;

// State management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static NFT_STORAGE: RefCell<StableBTreeMap<Principal, LivingNFT, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct LivingNFT {
    pub owner: Principal,
    pub name: String,
    pub creation_time: u64,
    pub personality: Personality,
    pub memories: Vec<Memory>,
    pub interaction_count: u64,
    pub growth_level: u32,
    pub last_interaction: u64,
}

// Canister methods
#[ic_cdk_macros::update]
pub async fn create_nft(name: String) -> Result<Principal> {
    let caller = ic_cdk::caller();
    let nft = LivingNFT {
        owner: caller,
        name,
        creation_time: ic_cdk::api::time(),
        personality: Personality::generate_initial(),
        memories: Vec::new(),
        interaction_count: 0,
        growth_level: 1,
        last_interaction: ic_cdk::api::time(),
    };

    let nft_id = ic_cdk::id();
    NFT_STORAGE.with(|storage| {
        storage.borrow_mut().insert(nft_id, nft)
    });

    Ok(nft_id)
}

#[ic_cdk_macros::query]
pub fn get_nft(id: Principal) -> Result<LivingNFT> {
    NFT_STORAGE.with(|storage| {
        storage.borrow().get(&id)
            .ok_or(NftError::NotFound)
    })
}

#[ic_cdk_macros::update]
pub async fn interact(id: Principal, input: String) -> Result<InteractionResponse> {
    let caller = ic_cdk::caller();
    
    // Verify ownership
    let mut nft = get_nft(id)?;
    if nft.owner != caller {
        return Err(NftError::NotAuthorized);
    }

    // Process interaction
    let response = process_interaction(&mut nft, &input).await?;
    
    // Update storage
    NFT_STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, nft)
    });

    Ok(response)
}

// Helper function for processing interactions
async fn process_interaction(nft: &mut LivingNFT, input: &str) -> Result<InteractionResponse> {
    // Update interaction count and time
    nft.interaction_count += 1;
    nft.last_interaction = ic_cdk::api::time();

    // TODO: Implement LLM integration
    // For MVP, return simple response
    Ok(InteractionResponse {
        message: format!("Interaction processed! Count: {}", nft.interaction_count),
        personality_changes: Vec::new(),
        new_memories: Vec::new(),
    })
}