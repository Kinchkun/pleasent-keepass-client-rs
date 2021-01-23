use serde::Deserialize;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CredentialEntry {
    pub id: String,
    pub name: String,
    pub username: Option<String>,
    pub notes: Option<String>,
    pub group_id: String,
    pub created: DateTime,
    pub modified: DateTime,
    pub expires: Option<DateTime>,
    pub synced: Option<DateTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Folder {
    pub credentials: Vec<CredentialEntry>,
    pub children: Vec<Folder>,
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub created: DateTime,
    pub modified: DateTime,
}
