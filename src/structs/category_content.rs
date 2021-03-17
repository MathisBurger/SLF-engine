use crate::structs::letter_entry::LetterEntry;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct CategoryContent {
    pub a: Vec<LetterEntry>
}