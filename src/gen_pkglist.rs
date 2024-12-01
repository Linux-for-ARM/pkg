//! Generate `md/pkgs.md` file

use crate::pkg_list::{PkgList, PkgMeta};

use std::fs;
use std::io::Result;
use std::path::Path;

pub fn gen_md_content(pkg: &str, meta: &PkgMeta) -> String {
    format!(
        "## {pkg}-{}\n\
        {}\n\
        - Домашняя страница: <{}>\n\
        - Скачать: <{}>\n\
        - MD5-сумма: `{}`\n",
        &meta.version, &meta.description, &meta.home_page, &meta.download, &meta.md5,
    )
}

pub fn generate<P: AsRef<Path>>(root_dir: P, pkg_list: &PkgList) -> Result<()> {
    let pkgs_md = root_dir.as_ref().join("pkgs.md");
    let mut s = String::new();

    for pkg in &pkg_list.package {
        s = format!("{s}{}", gen_md_content(pkg.0, pkg.1));
    }

    fs::write(pkgs_md, s)
}