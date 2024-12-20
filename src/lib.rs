use candid::{CandidType, Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

// Type aliases
type Memory = VirtualMemory<DefaultMemoryImpl>;

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
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Personality {
    pub empathy: f32,
    pub curiosity: f32,
    pub resilience: f32,
    pub synthesis: f32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Memory {
    pub timestamp: u64,
    pub content: String,
    pub emotional_impact: f32,
}

// Canister methods
#[ic_cdk_macros::update]
pub async fn create_nft(name: String) -> Result<Principal, String> {
    let caller = ic_cdk::caller();
    let nft = LivingNFT {
        owner: caller,
        name,
        creation_time: ic_cdk::api::time(),
        personality: generate_initial_personality(),
        memories: Vec::new(),
        interaction_count: 0,
        growth_level: 1,
    };

    let nft_id = ic_cdk::id();
    NFT_STORAGE.with(|storage| {
        storage.borrow_mut().insert(nft_id, nft)
    });

    Ok(nft_id)
}

#[ic_cdk_macros::query]
pub fn get_nft(id: Principal) -> Option<LivingNFT> {
    NFT_STORAGE.with(|storage| {
        storage.borrow().get(&id)
    })
}

// Helper functions
fn generate_initial_personality() -> Personality {
    Personality {
        empathy: 0.5,
        curiosity: 0.7,
        resilience: 0.6,
        synthesis: 1.0,  // Genesis trait
    }
}