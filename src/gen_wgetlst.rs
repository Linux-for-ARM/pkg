//! Генерация файла `wget-list`

use crate::pkg_list::PkgList;
use lfa_rs::prelude::*;
use lfa_rs::utils::write;

use std::path::Path;

fn get_wgetlst_content(pkg_list: &PkgList) -> String {
    let mut wgetlst = String::new(); // строка типа {url}\n{url}\n...
    let mut strs = vec![];
    for pkg in pkg_list.package.keys() {
        // Получаем URL пакета и добавляем его к строке wgetlst
        let url = &pkg_list.package[pkg].download;
        let file = url.rsplit_once('/').unwrap_or(("", "")).1;

        // wgetlst = format!("{wgetlst}{url}\n");
        strs.push((url, file));
    }
    // Для сортировки по имени файла, а не по всему URL
    strs.sort_by(|a, b| a.1.cmp(b.1));

    for s in strs {
        wgetlst = format!("{wgetlst}{}\n", s.0);
    }

    wgetlst.trim().to_string()
}

pub fn gen_wgetlst_file<P: AsRef<Path>>(pkg_list: &PkgList, path: P) -> Result<()> {
    let wgetlst = get_wgetlst_content(pkg_list);
    // dbg!(&wgetlst);
    write(&path, wgetlst)?;
    Ok(())
}
