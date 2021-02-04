use crate::model::http_types::*;
use crate::PleasantResult;
use chrono::Utc;
use log::*;
use rusqlite::{params, Connection};
use std::fmt::Formatter;
use std::path::PathBuf;

/// stores credentials entries into the sql database.
pub struct CredentialsDatabase {
    connection: Connection,
}

pub enum DatabasePath {
    File(PathBuf),
    InMem,
}

impl std::fmt::Display for DatabasePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabasePath::File(path) => write!(f, "Path: {}", path.display()),
            DatabasePath::InMem => write!(f, "In Memory (changes will be lost)"),
        }
    }
}

impl CredentialsDatabase {
    pub fn new(path: DatabasePath) -> PleasantResult<Self> {
        info!("Open database at: {}", path);

        let connection = match path {
            DatabasePath::File(path) => Connection::open(path),
            DatabasePath::InMem => Connection::open_in_memory(),
        }?;

        debug!("Opening database was successful");
        Ok(CredentialsDatabase { connection })
    }
    pub fn add_root_folder(&self, folder: Folder) -> PleasantResult<()> {
        debug!("Add root folder. Truncating tables");
        self.connection.execute_batch(
            r#"
DELETE FROM credentials;
DELETE FROM folders;
DELETE FROM attachments;
        "#,
        )?;

        self.add_folder(folder)
    }

    fn add_folder(&self, folder: Folder) -> PleasantResult<()> {
        debug!("Add folder {}", &folder.name);

        let statement = r#"
INSERT INTO folders (id, parent_id, name, created, modified, expires, synced) 
VALUES (?1,?2,?3,?4,?5,?6,?7)
"#;
        let id = &folder.id;
        let parent_id = &folder.parent_id;
        let name = &folder.name;
        let created = &folder.created;
        let modified = &folder.modified;
        let expires = &folder.expires;
        let synced = Utc::now();
        self.connection.execute(
            statement,
            params![id, parent_id, name, created, modified, expires, synced],
        )?;

        for cred in folder.credentials.into_iter() {
            self.add_credentials(cred)?;
        }

        for child_folder in folder.children.into_iter() {
            self.add_folder(child_folder)?;
        }

        Ok(())
    }

    fn add_credentials(&self, credential: CredentialEntry) -> PleasantResult<()> {
        use colored::{ColoredString, Colorize};
        debug!("Add credentials entry ({})", &credential.name.blue());
        let id = &credential.id;
        let name = &credential.name;
        let username = &credential.username;
        let notes = &credential.notes;
        let group_id = &credential.group_id;
        let created = &credential.created;
        let modified = &credential.modified;
        let expires = &credential.expires;
        let synced = Utc::now();
        self.connection.execute(
            r#"
INSERT INTO credentials (id, name, username, notes, group_id, created, modified, expires, synced) 
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
"#,
            params![id, name, username, notes, group_id, created, modified, expires, synced],
        )?;

        for attachment in credential.attachments.into_iter() {
            self.add_attachment(attachment)?;
        }

        Ok(())
    }

    fn add_attachment(&self, attachment: Attachment) -> PleasantResult<()> {
        debug!("Add attachment into database");
        let statement = r#"
INSERT INTO attachments (id, credentials_id, file_name, file_size) 
VALUES (?1,?2,?3,?4)
"#;
        let id = &attachment.attachment_id;
        let credentials_id = &attachment.credential_object_id;
        let file_name = &attachment.file_name;
        let file_size = &attachment.file_size;

        self.connection
            .execute(statement, params![id, credentials_id, file_name, file_size])?;

        Ok(())
    }

    fn init_db(&self) -> PleasantResult<()> {
        debug!("Initialize credentials database");
        let sql_statement = include_str!("../../assets/sql/init_db.sql");
        self.connection.execute_batch(sql_statement)?;
        Ok(())
    }
}
