use crate::data_structures::search_index::SearchIndex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchIndices {
    pub card_identifier_to_card_id: SearchIndex<String, i32>,
    pub set_identifier_to_set_id: SearchIndex<String, i32>,
}