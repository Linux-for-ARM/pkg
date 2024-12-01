//! Методы для получения списка пакетов, которые используются в LFA

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use lfa_rs::traits::Toml;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgList {
    pub package: HashMap<String, PkgMeta>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PkgMeta {
    pub version: String,
    pub description: String,
    pub home_page: String,
    pub download: String,
    pub md5: String,
    pub display: Option<bool>,
}

impl Toml for PkgList {}
impl Toml for PkgMeta {}
