use super::prelude::*;
use super::sub_wins::is_exit_win;
use crate::pkg_list::PkgList;
use crate::pkg_list::PkgMeta;

use cursive::traits::{Nameable, Resizable};
use cursive::view::Scrollable;
use cursive::views::{EditView, ListView, SelectView};

use lfa_rs::traits::Toml;

pub fn list_win(scr: &mut Cursive) {
    scr.pop_layer();
}

pub fn add_package_to_list_win(scr: &mut Cursive) {
    let pkg_list = ListView::new()
        .child("Имя:", EditView::new().with_name("pkg_list.name"))
        .child("Версия:", EditView::new().with_name("pkg_list.version"))
        .child(
            "Домашняя страница:",
            EditView::new().with_name("pkg_list.home_page"),
        )
        .child(
            "Архив (URL):",
            EditView::new().with_name("pkg_list.download"),
        )
        .child(
            "Описание:",
            EditView::new()
                .with_name("pkg_list.description")
                .min_width(40),
        );

    let layout = LinearLayout::vertical()
        .child(TextView::new(
            "Данный пакет будет добавлен только в список пакетов. Для\n\
                      каждого использующегося пакета это нужно сделать один раз.",
        ))
        .child(Panel::new(pkg_list));

    let win = Dialog::around(layout)
        .title("Добавление пакета в СПИСОК")
        .button("OK", |s| {
            s.pop_layer();
        })
        .button("Выход", super::sub_wins::is_exit_win);

    scr.add_layer(win);
}

pub fn add_package_to_index_win(scr: &mut Cursive) {
    let pkg_info = ListView::new()
        .child("Markdown ID:", EditView::new().with_name("pkg_info.id"))
        .child(
            "ID из списка:",
            EditView::new().with_name("pkg_info.package"),
        )
        .child("ОВС:", EditView::new().with_name("pkg_info.sbu"))
        .child("dir_pth:", EditView::new().with_name("pkg_info.dir_pth"))
        .child(
            "Описаниеv (опционально):",
            EditView::new()
                .with_name("pkg_info.description")
                .min_width(40),
        );

    let layout = LinearLayout::vertical()
        .child(TextView::new(
            "Добавление пакета в индекс пакетов. Если пакет\n\" ",
        ))
        .child(Panel::new(pkg_info));

    let win = Dialog::around(layout)
        .title("Добавление пакета в ИНДЕКС")
        .button("OK", |s| {
            s.pop_layer();
        })
        .button("Выход", super::sub_wins::is_exit_win);

    scr.add_layer(win);
}

pub fn pkg_list_win(scr: &mut Cursive) {
    let pkgs = scr.user_data::<PkgList>().unwrap();
    let mut pkg_list = SelectView::new().autojump();
    pkg_list.set_on_submit(on_selected_pkg_from_list);

    let mut packages = Vec::new(); // содержит данные из хеш-карты (имя и версия пакета) для отображения в меню
    for pkg in pkgs.package.keys() {
        packages.push((
            format!("{}-{}", pkg, pkgs.package[pkg].version),
            pkg.to_string(),
        ));
    }
    packages.sort(); // в хеш-карте данные не сортированы. Вектор мы можем отсортировать
                     // для более удобного отображения данных из него в главном меню

    // Добавляем полученные данные (пара {"имя-версия пакета", "имя-пакета"}) в меню
    for pkg in packages {
        pkg_list.add_item(pkg.0, pkg.1);
    }

    let win = Dialog::around(Panel::new(pkg_list.scrollable()))
        .title("Список всех пакетов")
        .button("Добавить", |_| {})
        .button("Сохранить", |s| write_to_disk(s))
        .button("Выйти", |s| is_exit_win(s));
    scr.add_layer(win);
}

fn on_selected_pkg_from_list(scr: &mut Cursive, pkg: &String) {
    let package = scr
        .user_data::<PkgList>()
        .unwrap()
        .package
        .get(pkg)
        .unwrap();

    let text = ListView::new()
        .child("Name:", TextView::new(pkg))
        .child(
            "Version:",
            EditView::new()
                .content(&package.version)
                .with_name("package.version")
                .min_width(80),
        )
        .child(
            "Description:",
            EditView::new()
                .content(&package.description)
                .with_name("package.description"),
        )
        .child(
            "Home page:",
            EditView::new()
                .content(&package.home_page)
                .with_name("package.home_page"),
        )
        .child(
            "Download URL:",
            EditView::new()
                .content(&package.download)
                .with_name("package.download"),
        );

    let pkg = pkg.to_string();

    scr.add_layer(
        Dialog::around(text)
            .title("Редактирование информации")
            .button("Сохранить в ОЗУ", move |s| {
                write_metadata(s, pkg.clone());
                s.pop_layer();
            })
            .button("Закрыть без сохранения", |s| {
                s.pop_layer();
            }),
    );
}

fn write_metadata(scr: &mut Cursive, pkg: String) {
    let mut pkgs = scr.take_user_data::<PkgList>().unwrap();
    let old_pkg_meta = pkgs.package.get(&pkg).unwrap();
    let pkg_meta = PkgMeta {
        version: scr
            .call_on_name("package.version", |s: &mut EditView| {
                s.get_content().to_string()
            })
            .unwrap(),
        description: scr
            .call_on_name("package.description", |s: &mut EditView| {
                s.get_content().to_string()
            })
            .unwrap(),
        home_page: scr
            .call_on_name("package.home_page", |s: &mut EditView| {
                s.get_content().to_string()
            })
            .unwrap(),
        download: scr
            .call_on_name("package.download", |s: &mut EditView| {
                s.get_content().to_string()
            })
            .unwrap(),
        ..old_pkg_meta.clone()
    };
    pkgs.package.insert(pkg, pkg_meta);
    scr.set_user_data(pkgs);
}

fn write_to_disk(scr: &mut Cursive) {
    let pkgs = scr.take_user_data::<PkgList>().unwrap();
    match pkgs.write("pkg-packages.toml") {
        Ok(_) => {
            scr.add_layer(
                Dialog::around(TextView::new(
                    "Запись произведена успешно («pkg-packages.toml»). Для обновления\n\
                              отображаемой информации перезайдите в pkg. Также вам нужно будет\n\
                              обновить контрольные суммы изменённых пакетов (`pkg update-md5 -p\n\
                              METADATA.toml`).",
                    // Обновление контрольных сумм в метаданных (package.toml)
                    // нужно, поскольку пользователь мог обновить URL для
                    // скачивания исходников пакета, в таком случае могла
                    // обновиться контрольная сумма этого архива. Для получения
                    // актуальных сведений о контрольных суммах их нужно будет
                    // обновить.
                ))
                .title("Запись изменений на диск")
                .button("ОК", |s| {
                    s.pop_layer();
                }),
            );
        }
        Err(why) => {
            scr.add_layer(
                Dialog::around(TextView::new(format!(
                    "ОШИБКА записи изменений в файл «pkg-packages.toml»:\n\n{why}"
                )))
                .title("Запись изменений на диск")
                .button("ОК", |s| {
                    s.pop_layer();
                }),
            );
        }
    }
}
