use crate::db::db_types::{Attachment, CredentialEntry, Folder};
use crate::types::*;
use chrono::Utc;
use log::*;
use rusqlite::{params, Connection};

pub struct PleasantPasswordModel {
    connection: Connection,
}

impl PleasantPasswordModel {
    pub fn new(connection: Connection) -> Result<Self> {
        let model = PleasantPasswordModel { connection };
        model.init_db()?;
        Ok(model)
    }

    pub fn add_root_folder(&self, folder: Folder) -> Result<()> {
        debug!("Add root folder. Truncating tables");
        self.connection
            .execute("DELETE FROM credentials;", params![])?;
        self.connection.execute("DELETE FROM folders;", params![])?;

        self.add_folder(folder)
    }

    fn add_folder(&self, folder: Folder) -> Result<()> {
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

    fn add_credentials(&self, credential: CredentialEntry) -> Result<()> {
        debug!("Add credentials entry {}", &credential.name);
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

    fn add_attachment(&self, attachment: Attachment) -> Result<()> {
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

    fn init_db(&self) -> Result<()> {
        debug!("Initialize credentials database");
        let sql_statement = include_str!("../assets/sql/init_db.sql");
        self.connection.execute_batch(sql_statement)?;
        Ok(())
    }
}
