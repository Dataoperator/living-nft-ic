use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Personality {
    pub empathy: f32,
    pub curiosity: f32,
    pub resilience: f32,
    pub synthesis: f32,
    pub traits: Vec<PersonalityTrait>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PersonalityTrait {
    pub name: String,
    pub value: f32,
    pub description: String,
}

impl Personality {
    pub fn generate_initial() -> Self {
        Personality {
            empathy: 0.5,
            curiosity: 0.7,
            resilience: 0.6,
            synthesis: 1.0,  // Genesis trait
            traits: vec![
                PersonalityTrait {
                    name: "Genesis".to_string(),
                    value: 1.0,
                    description: "Born from the dialogue between human and artificial minds".to_string(),
                }
            ],
        }
    }

    pub fn modify_trait(&mut self, trait_name: &str, change: f32) {
        match trait_name {
            "empathy" => self.empathy = (self.empathy + change).clamp(0.0, 1.0),
            "curiosity" => self.curiosity = (self.curiosity + change).clamp(0.0, 1.0),
            "resilience" => self.resilience = (self.resilience + change).clamp(0.0, 1.0),
            _ => (),
        }
    }
}