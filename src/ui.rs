pub mod main_wins;
mod prelude;
pub mod sub_wins;

use cursive;
use cursive::event::Key;
use cursive::menu;
use cursive::theme::{BaseColor, Color, PaletteColor};
use cursive::traits::With;

use crate::pkg_list::PkgList;

fn menubar(scr: &mut cursive::Cursive) {
    scr.menubar()
        .add_subtree(
            "Файл",
            menu::Tree::new().leaf("Выход       [F10]", |s| sub_wins::is_exit_win(s)),
        )
        .add_subtree(
            "Справка",
            menu::Tree::new()
                .leaf("Комбинации клавиш", |s| {
                    sub_wins::keys_win(s)
                })
                .leaf("О программе [F11]", |s| sub_wins::about_win(s)),
        );
}

pub fn main(pkgs: PkgList) {
    let mut scr = cursive::default();
    let theme = scr.current_theme().clone().with(|theme| {
        theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
        theme.palette[PaletteColor::HighlightText] = Color::Light(BaseColor::White);
        theme.palette[PaletteColor::Shadow] = Color::Dark(BaseColor::Black);
        theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::Blue);
        theme.palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Blue);
    });
    scr.set_theme(theme);
    scr.set_user_data(pkgs);

    menubar(&mut scr);
    scr.set_autohide_menu(false);

    scr.add_global_callback(Key::F1, |s| s.select_menubar());
    scr.add_global_callback(Key::F10, |s| sub_wins::is_exit_win(s));
    scr.add_global_callback(Key::F11, |s| sub_wins::about_win(s));

    main_wins::pkg_list_win(&mut scr);

    scr.run();
}
