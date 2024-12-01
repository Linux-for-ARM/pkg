pub mod gen_md5;
pub mod gen_pkginfo;
pub mod gen_pkglist;
pub mod gen_wgetlst;
pub mod pkg_info;
pub mod pkg_list;
pub mod ui;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    /// Генерирует файл с контрольными md5-суммами
    GenMd5 {
        /// Путь до файла с описанием пакетов
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../packages.toml"))]
        pkg_list: String,

        /// Путь до файла, в который будут записаны контрольные суммы
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../md5sums"))]
        md5sums: String,
    },
    /// Генерирует файл `wget-list`
    GenWget {
        /// Путь до файла с описанием пакетов
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../packages.toml"))]
        pkg_list: String,

        /// Путь до файла, в который будут записаны URL
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../wget-list"))]
        wget_list: String,
    },
    /// Интерактивное редактирование метаданных (`package.toml`)
    Index {
        /// Путь до файла с описанием пакетов
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../packages.toml"))]
        pkg_list: String,
    },
    /// Генерирует md-файлы с информацией о пакетах
    Info {
        /// Путь до директории с md-файлами руководства
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../md"))]
        md: String,

        /// Путь до файла с описанием пакетов
        #[arg(long)]
        #[arg(default_value_t = String::from("../packages.toml"))]
        pkg_list: String,

        /// Путь до файла со списком пакетов
        #[arg(long)]
        #[arg(default_value_t = String::from("../pkg_info.toml"))]
        pkg_info: String,
    },
    /// Генерирует файл `md/pkgs.md`
    List {
        /// Путь до директории с md-файлами руководства
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../md"))]
        md: String,

        /// Путь до файла с описанием пакетов
        #[arg(long)]
        #[arg(default_value_t = String::from("../packages.toml"))]
        pkg_list: String,
    },
    /// Выполняет полную генерацию всех файлов (исключая режим `index`)
    All {
        /// Путь до директории с md-файлами руководства
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../md"))]
        md: String,

        /// Путь до файла с описанием пакетов
        #[arg(long)]
        #[arg(default_value_t = String::from("../packages.toml"))]
        pkg_list: String,

        /// Путь до файла со списком пакетов
        #[arg(long)]
        #[arg(default_value_t = String::from("../pkg_info.toml"))]
        pkg_info: String,

        /// Путь до файла, в который будут записаны URL
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../wget-list"))]
        wget_list: String,

        /// Путь до файла, в который будут записаны контрольные суммы
        #[arg(short, long)]
        #[arg(default_value_t = String::from("../md5sums"))]
        md5sums: String,
    },
}
