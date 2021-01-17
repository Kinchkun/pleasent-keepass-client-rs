use colored::{Color, ColoredString, Colorize};
use log::*;
use serde::export::Formatter;
use std::env;
use std::env::VarError;
use std::fmt::{Debug, Display, Result as FmtResult};
use url::{ParseError, Url};

/// A string but will be masked when printed
pub struct SecureString(String);

impl Debug for SecureString {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[MASKED]")
    }
}

impl Display for SecureString {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[MASKED]")
    }
}

impl SecureString {
    pub fn red(&self) -> ColoredString {
        self.to_string().red()
    }
    pub fn blue(&self) -> ColoredString {
        self.to_string().blue()
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Loads a required setting entry as string from the environment
///
/// Panics if not present.
pub fn require_string<S: AsRef<str>>(setting_name: S) -> String {
    let setting_name = setting_name.as_ref();
    let setting_value = load_setting(setting_name);
    info!(
        "Successfully load {} with value {}",
        setting_name.blue(),
        setting_value.blue()
    );
    setting_value
}

/// Loads a required setting entry as SecureString from the environment
///
/// Panics if not present.
pub fn require_secure_string<S: AsRef<str>>(setting_name: S) -> SecureString {
    let setting_name = setting_name.as_ref();
    let setting_value = SecureString(load_setting(setting_name));
    info!(
        "Successfully load {} with value {}",
        setting_name.blue(),
        setting_value.blue()
    );
    setting_value
}

/// Loads a required setting entry as url from the environment
///
/// Panics if not present or not a valid url.
pub fn require_url<S: AsRef<str>>(setting_name: S) -> Url {
    let setting_name = setting_name.as_ref();
    let setting_value = load_setting(setting_name);
    match setting_value.parse::<Url>() {
        Ok(result) => {
            info!(
                "Successfully load url {} with value {}",
                setting_name.blue(),
                setting_value.to_string().blue()
            );
            return result;
        }
        Err(err) => {
            eprintln!(
                "Could not parse setting {}. It is present but not a valid URL. Value: {}. Error: {}",
                setting_name.blue(),
                setting_value.blue(),
                err.to_string().red()
            );
            std::process::exit(1);
        }
    }
}

/// Loads a required setting entry from the environment
///
/// Panics if not present.
fn load_setting(setting_name: &str) -> String {
    debug!("Load setting {}", setting_name);
    match env::var(setting_name) {
        Ok(result) => result,
        Err(err) => {
            match err {
                VarError::NotPresent => {
                    eprintln!(
                        r#"ERROR: Could not load setting {} because it is not present.
Please set it as environment variable or add it to the .env file. E.g.:
    {}="value""#,
                        setting_name.blue(),
                        setting_name
                    );
                }
                VarError::NotUnicode(_) => {
                    eprintln!(
                        "Could not load setting {} because it is not a valid utf8 string.",
                        setting_name
                    )
                }
            }
            std::process::exit(1);
        }
    }
}
