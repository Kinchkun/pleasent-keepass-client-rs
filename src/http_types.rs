use serde::Deserialize;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CredentialEntry {
    pub id: String,
    pub name: String,
    pub username: Option<String>,
    pub attachments: Vec<Attachment>,
    pub notes: Option<String>,
    pub group_id: String,
    pub created: DateTime,
    pub modified: DateTime,
    pub expires: Option<DateTime>,
    pub synced: Option<DateTime>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Folder {
    pub credentials: Vec<CredentialEntry>,
    pub children: Vec<Folder>,
    pub id: String,
    pub name: String,
    pub parent_id: String,
    pub created: DateTime,
    pub modified: DateTime,
    pub expires: Option<DateTime>,
    pub synced: Option<DateTime>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    pub credential_object_id: String,
    pub attachment_id: String,
    pub file_name: String,
    pub file_size: i64,
}
