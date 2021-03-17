use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LetterEntry {
    pub name: String,
    pub description: String,
    pub source: String
}