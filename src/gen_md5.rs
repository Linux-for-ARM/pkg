//! Генерация файла `md5sums`

use crate::pkg_list::PkgList;
use lfa_rs::prelude::*;
use lfa_rs::utils::write;

use std::path::Path;

fn get_md5sums_content(pkg_list: &PkgList) -> String {
    // строка с контрольными суммами не такая большая, так что всё норм
    let mut md5sums = String::new();
    let mut strs = vec![];

    for pkg in pkg_list.package.keys() {
        let url = &pkg_list.package[pkg].download;
        let md5 = &pkg_list.package[pkg].md5;

        let file = url.rsplit_once('/').unwrap_or(("", "")).1;
        strs.push((md5, file));
    }
    // Для сортировки по имени файла, а не md-сумме
    strs.sort_by(|a, b| a.1.cmp(b.1));

    for s in strs {
        md5sums = format!("{md5sums}{}  {}\n", s.0, s.1);
    }

    md5sums.trim().to_string()
}

pub fn gen_md5sums_file<P: AsRef<Path>>(pkg_list: &PkgList, path: P) -> Result<()> {
    let md5sums = get_md5sums_content(pkg_list);
    write(&path, md5sums)?;
    Ok(())
}
