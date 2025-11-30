//! The file contains definitions of domain entries as serde-serializable models.


use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EntryModel {
    pub title:       String,
    pub command:     String,
    pub description: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EntryModelStorageDef {
    pub entry: Vec<EntryModel>,
}
