use chrono::Utc;
use log::*;
use rusqlite::{params, Connection};
use serde::Serialize;

use crate::model::http_types::{Attachment, CredentialEntry, Folder};
use crate::types::*;

pub struct PleasantPasswordModel<'a> {
    connection: &'a Connection,
}

#[derive(Debug, Serialize, PartialEq, Default)]
pub struct Credentials {
    pub id: String,
    pub folder_name: String,
    pub name: String,
    pub username: Option<String>,
    pub notes: Option<String>,
}

impl<'a> PleasantPasswordModel<'a> {
    // pub fn new(connection: &'a Connection) -> PleasantResult<Self> {
    //     let model = PleasantPasswordModel { connection };
    //     model.init_db()?;
    //     Ok(model)
    // }

    pub fn query_for_credentials(&self, query: &str) -> PleasantResult<Vec<Credentials>> {
        let mut stmt = self.connection.prepare(
            r#"
SELECT c.id, f.name, c.name, c.username, c.notes FROM credentials c
INNER JOIN folders f on c.group_id = f.id
WHERE f.name like '%' || :query || '%'
OR c.name like '%' || :query || '%'
OR c.username like '%' || :query || '%'
OR c.notes like '%' || :query || '%'
"#,
        )?;

        let mut rows = stmt.query_named(&[(":query", &query)])?;

        let mut result: Vec<Credentials> = Vec::new();
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let folder_name: String = row.get(1)?;
            let name: String = row.get(2)?;
            let username: Option<String> = row.get(3)?;
            let notes: Option<String> = row.get(4)?;

            result.push(Credentials {
                id,
                folder_name,
                name,
                username,
                notes,
            });
        }

        Ok(result)
    }
}
