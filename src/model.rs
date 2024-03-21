use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CombinationResult {
    pub result: String,
    pub emoji: String,
    #[serde(rename = "isNew")]
    pub is_new: bool,
}

#[derive(Debug)]
pub struct InfiniteCraft {
    pub ingr1: String,
    pub ingr2: String,
    pub out: String,
}
