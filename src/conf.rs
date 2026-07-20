use std::{io, net::Ipv4Addr, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConf {
    pub(crate) db: DBConf,
    pub(crate) http: HttpConf,
    pub(crate) images: ImagesConf,
}

#[derive(Deserialize)]
pub(crate) struct DBConf {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) name: String,
    pub(crate) app_user: DBUser,
    pub(crate) migration_user: DBUser,
}

#[derive(Deserialize)]
pub(crate) struct DBUser {
    pub(crate) name: String,
    pub(crate) password: String,
}

#[derive(Deserialize)]
pub(crate) struct ImagesConf {
    pub(crate) storage_root: String,
    pub(crate) url_prefix: String,
}

#[derive(Deserialize)]
pub(crate) struct HttpConf {
    pub(crate) ip: Ipv4Addr,
    pub(crate) port: u16,
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub(crate) enum AppConfParsingError {
    IO(#[from] io::Error),
    Deserialize(#[from] toml::de::Error),
}

impl AppConf {
    pub(crate) fn from_file(path: PathBuf) -> Result<Self, AppConfParsingError> {
        let file_content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&file_content)?)
    }
}

//TODO add unit tests
