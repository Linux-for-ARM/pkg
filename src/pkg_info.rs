//! Методы для получения сведений о конкретном пакете в конкретном расположении в LFA

use serde::Serialize;
use serde::Deserialize;

use lfa_rs::traits::Toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct PkgInfo {
    pub package: Vec<PkgMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PkgMeta {
    /// Имя созданного md-файла
    pub id: String,

    /// Имя пакета из файла packages.toml (crate::pkg_list::PkgList)
    pub package: String,

    /// Опциональное описание (имеет более высокий приоритет, чем описание из PkgList)
    pub description: Option<String>,

    /// ОВС
    pub sbu: f32,

    /// В какой директории сохранять файл описания пакета
    pub dir_pth: String,

    /// Опциональный список патчей
    pub patch: Option<Vec<String>>,
}

impl Toml for PkgInfo {}
impl Toml for PkgMeta {}
