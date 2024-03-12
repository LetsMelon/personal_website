use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RefType {
    Tag,
    Branch,
}

#[derive(Debug, Deserialize)]
pub struct GitHubWebhook {
    // description: String,
    // master_branch: String,
    // TODO enum
    pub pusher_type: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub ref_type: RefType,
    // repository: serde_json::Value,
    // sender: serde_json::Value,
}
