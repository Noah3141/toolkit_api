
use serde::{Serialize, Deserialize};

// INCOMING DATA
#[derive(FromForm, Deserialize)]
pub struct GenerateListRequest {
    input: String,
    breadth: String,
    style: String,
}

// OUTGOING DATA
#[derive(Serialize)]
pub struct GenerateListResponse {
    list: Vec<VocabEntry>,
    style: String,
    metadata: String
}

#[derive(Serialize)]
pub struct VocabEntry {
    form: Option<String>,
    lemma: Option<String>,

    imperfective: Option<String>,
    perfective: Option<String>,
    
    stem: Option<String>,
    prefixes: Option<Vec<String>>,

    frequency: u16,
    part_of_speech: Option<String>
}