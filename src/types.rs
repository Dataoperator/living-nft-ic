use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InteractionResponse {
    pub message: String,
    pub personality_changes: Vec<PersonalityChange>,
    pub new_memories: Vec<Memory>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PersonalityChange {
    pub trait_name: String,
    pub old_value: f32,
    pub new_value: f32,
    pub reason: String,
}
