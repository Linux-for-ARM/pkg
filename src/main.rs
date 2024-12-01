use lfa_rs::error::Result;
use lfa_rs::traits::Toml;

use pkg::gen_md5::gen_md5sums_file;
use pkg::gen_pkginfo;
use pkg::gen_pkglist;
use pkg::gen_wgetlst::gen_wgetlst_file;
use pkg::pkg_info::PkgInfo;
use pkg::pkg_list::PkgList;
use pkg::ui;
use pkg::Cli;
use pkg::Mode;

use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.mode {
        Mode::GenMd5 { md5sums, pkg_list } => {
            let pkg_list = PkgList::parse(&pkg_list)?;
            gen_md5sums_file(&pkg_list, &md5sums)?;
        }
        Mode::GenWget {
            wget_list,
            pkg_list,
        } => {
            let pkg_list = PkgList::parse(&pkg_list)?;
            gen_wgetlst_file(&pkg_list, &wget_list)?;
        }
        Mode::Index { pkg_list } => {
            let pkg_list = PkgList::parse(&pkg_list)?;
            ui::main(pkg_list.clone())
        }
        Mode::Info {
            md,
            pkg_list,
            pkg_info,
        } => {
            let pkg_list = PkgList::parse(&pkg_list).unwrap();
            let pkg_info = PkgInfo::parse(&pkg_info).unwrap();
            gen_pkginfo::generate(md, &pkg_list, &pkg_info).unwrap();
        }
        Mode::List { md, pkg_list } => {
            let pkg_list = PkgList::parse(&pkg_list).unwrap();
            gen_pkglist::generate(md, &pkg_list).unwrap();
        }
        Mode::All { md, pkg_list, pkg_info, wget_list, md5sums } => {
            let pkg_list = PkgList::parse(&pkg_list).unwrap();
            let pkg_info = PkgInfo::parse(&pkg_info).unwrap();

            println!("Update md5sums...");
            gen_md5sums_file(&pkg_list, &md5sums)?;

            println!("Update wget-list...");
            gen_wgetlst_file(&pkg_list, &wget_list)?;

            println!("Generate md/*/pkgs/*.md files...");
            gen_pkginfo::generate(&md, &pkg_list, &pkg_info).unwrap();

            println!("Generate md/pkgs.md file...");
            gen_pkglist::generate(&md, &pkg_list).unwrap();
        }
    }

    Ok(())
}
