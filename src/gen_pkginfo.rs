//! Generate Markdown pages with package description

use crate::pkg_info::{self, PkgInfo};
use crate::pkg_list::{self, PkgList};

use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

enum Dirs {
    CrossCompiler,
    Base,
    Kernel,
    Bootloader,
    BootFiles,
}

impl Dirs {
    pub const ALL: [Self; 5] = [
        Self::CrossCompiler,
        Self::Base,
        Self::Kernel,
        Self::Bootloader,
        Self::BootFiles,
    ];

    pub fn from<P: AsRef<Path>>(pth: &str, root: P) -> PathBuf {
        let dirs = match pth {
            "cross-compiler" => Self::CrossCompiler,
            "base" => Self::Base,
            "kernel" => Self::Kernel,
            "bootloader" => Self::Bootloader,
            "boot-files" => Self::BootFiles,
            _ => panic!("Unknown dir_pth {pth}!"),
        };

        dirs.path(&root)
    }

    pub fn path<P: AsRef<Path>>(&self, root: P) -> PathBuf {
        root.as_ref().join(match self {
            Self::CrossCompiler => "cross-compiler/pkgs/",
            Self::Base => "base/pkgs/",
            Self::Kernel => "kernel/pkgs/",
            Self::Bootloader => "bootloader/pkgs/",
            Self::BootFiles => "boot-files/pkgs/",
        })
    }
}

fn create_dirs<P: AsRef<Path>>(root: P) -> Result<()> {
    let dirs = Dirs::ALL;

    for d in dirs {
        let pth = d.path(&root);
        if !pth.is_dir() {
            println!("create {} dir...", &pth.display());
            fs::create_dir_all(pth)?;
        }
    }

    Ok(())
}

fn get_md_string(pkg: &pkg_list::PkgMeta, pkg_idx: &pkg_info::PkgMeta) -> String {
    let descr = match &pkg_idx.description {
        Some(d) => d,
        None => &pkg.description,
    };

    let mut pkginfo = format!(
        "> {descr}\n\
        > - **Версия:** {}\n\
        > - **Домашняя страница:** <{}>\n\
        > - **Время сборки:** {} ОВС\n",
        &pkg.version, &pkg.home_page, pkg_idx.sbu,
    );

    if pkg_idx.patch.is_some() {
        pkginfo = format!("{pkginfo}> - **Необходимые патчи:**\n");
        let patch = &pkg_idx.patch;
        let patch = patch.clone().unwrap();
        for p in patch {
            pkginfo = format!("{pkginfo}\n>    - <{p}>");
        }
    }

    pkginfo
}

fn write<P: AsRef<Path>>(data: &str, pkg_id: &str, dir: P) -> Result<()> {
    let pth = dir.as_ref().join(format!("{pkg_id}.md"));
    fs::write(pth, data)
}

pub fn generate<P: AsRef<Path>>(root_dir: P, pkg_list: &PkgList, pkg_info: &PkgInfo) -> Result<()> {
    println!("create_dirs");
    create_dirs(&root_dir)?;

    for pkg in &pkg_info.package {
        let name = &pkg.id;
        let pth = Dirs::from(&pkg.dir_pth, &root_dir);
        println!("write package «{}/{name}»...", &pth.display());

        match pkg_list.package.get(name) {
                Some(package) => {
                    let md_str = get_md_string(package, pkg);
                    write(&md_str, name, &pth)?;
                }
                None => continue,
        }
    }

    Ok(())
}
