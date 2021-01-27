use serde::export::Formatter;
use std::error::Error;
use std::fmt::Result as FmtResult;
use std::string::ToString;
use strum_macros::Display as StrumDisplay;

#[derive(Debug, PartialEq, Eq)]
pub struct PleasantError {
    pub kind: Kind,
    pub message: String,
    pub context: String,
    pub hint: Option<String>,
}

pub type DynError = std::boxed::Box<dyn std::error::Error>;

#[derive(Debug, StrumDisplay, PartialEq, Eq)]
pub enum Kind {
    Unhandled,
    WrongCredentials,
}

impl std::error::Error for PleasantError {}

impl std::fmt::Display for PleasantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(
            f,
            "An error occurred while {}: {}. ",
            self.context, self.message
        )?;
        writeln!(f, "Error kind: {}", &self.kind);
        // if let Some(source) = &self.source {
        //     writeln!(f, "Cause: {}", self.source)
        // }?;
        if let Some(hint) = &self.hint {
            writeln!(f, "Hint: {}", hint)?;
        };
        Ok(())
    }
}
