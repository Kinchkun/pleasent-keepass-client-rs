use colored::{ColoredString, Colorize};
use log::*;
use std::env;
use std::env::VarError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use url::Url;

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

pub fn optional_string<S: AsRef<str>>(setting_name: S) -> Option<String> {
    let setting_name = setting_name.as_ref();
    let setting_value = try_load_setting(setting_name);
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

/// Loads an optional setting and if present converts into an url.
///
/// returns `None` if the setting is not present.
/// panics if the url conversion was unsuccessful
pub fn optional_url<S: AsRef<str>>(setting_name: S) -> Option<Url> {
    let setting_name = setting_name.as_ref();
    try_load_setting(setting_name)
        .map(|url_string| {
            match url_string.parse::<Url>() {
                Ok(result) => {
                    info!(
                        "Successfully load url {} with value {}",
                        setting_name.blue(),
                        url_string.to_string().blue()
                    );
                    result
                }
                Err(err) => {
                    eprintln!(
                        "Could not parse setting {}. It is present but not a valid URL. Value: {}. Error: {}",
                        setting_name.blue(),
                        url_string.blue(),
                        err.to_string().red()
                    );
                    std::process::exit(1);
                }
            }
        })
}
/// Loads a required setting entry as url from the environment
///
/// Panics if not present or not a valid url.
pub fn require_url<S: AsRef<str>>(setting_name: S) -> Url {
    let setting_name = setting_name.as_ref();
    optional_url(setting_name).expect(format!("Could not load setting {}", setting_name).as_str())
}

fn try_load_setting(setting_name: &str) -> Option<String> {
    debug!("Load setting {}", setting_name);
    match env::var(setting_name) {
        Ok(result) => Some(result),
        Err(err) => match err {
            VarError::NotPresent => None,
            VarError::NotUnicode(_) => {
                eprintln!(
                    "Could not load setting {} because it is not a valid utf8 string.",
                    setting_name
                );
                std::process::exit(1);
            }
        },
    }
}

/// Loads a required setting entry from the environment
///
/// Panics if not present.
fn load_setting(setting_name: &str) -> String {
    debug!("Load setting {}", setting_name);
    match try_load_setting(setting_name) {
        Some(value) => value,
        None => {
            eprintln!(
                r#"ERROR: Could not load setting {} because it is not present.
Please set it as environment variable or add it to the .env file. E.g.:
    {}="value""#,
                setting_name.blue(),
                setting_name
            );
            std::process::exit(1);
        }
    }
}
