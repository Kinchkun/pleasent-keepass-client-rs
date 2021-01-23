use serde::Deserialize;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CredentialEntry {
    id: String,
    name: String,
    username: Option<String>,
    notes: Option<String>,
    group_id: String,
    created: DateTime,
    modified: DateTime,
    expires: Option<DateTime>,
    synced: Option<DateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Folder {
    credentials: Vec<CredentialEntry>,
    children: Vec<Folder>,
    id: String,
    name: String,
    parent_id: String,
    created: DateTime,
    modified: DateTime,
}
