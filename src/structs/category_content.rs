use crate::structs::letter_entry::LetterEntry;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct CategoryContent {
    pub a: Vec<LetterEntry>,
    pub b: Vec<LetterEntry>,
    pub c: Vec<LetterEntry>,
    pub d: Vec<LetterEntry>,
    pub e: Vec<LetterEntry>,
    pub f: Vec<LetterEntry>,
    pub g: Vec<LetterEntry>,
    pub h: Vec<LetterEntry>,
    pub i: Vec<LetterEntry>,
    pub j: Vec<LetterEntry>,
    pub k: Vec<LetterEntry>,
    pub l: Vec<LetterEntry>,
    pub m: Vec<LetterEntry>,
    pub n: Vec<LetterEntry>,
    pub o: Vec<LetterEntry>,
    pub p: Vec<LetterEntry>,
    pub q: Vec<LetterEntry>,
    pub r: Vec<LetterEntry>,
    pub s: Vec<LetterEntry>,
    pub t: Vec<LetterEntry>,
    pub u: Vec<LetterEntry>,
    pub v: Vec<LetterEntry>,
    pub w: Vec<LetterEntry>,
    pub x: Vec<LetterEntry>,
    pub y: Vec<LetterEntry>,
    pub z: Vec<LetterEntry>

}