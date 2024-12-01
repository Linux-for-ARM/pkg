use super::prelude::*;

pub fn keys_win(scr: &mut Cursive) {
    let layout = LinearLayout::vertical()
        .child(TextView::new("[F1 ] Select menubar"))
        .child(TextView::new("[F10] Exit"))
        .child(TextView::new("[F11] О программе"));

    let win = Dialog::around(layout)
        .title("Комбинации клавиш")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}

pub fn about_win(scr: &mut Cursive) {
    let layout = LinearLayout::vertical()
        .child(TextView::new(format!(
            "{} ver.{} (C) 2024 {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )))
        .child(DummyView)
        .child(TextView::new(
            "Над программой в одиночку работает студент из России. Вы\n\
                      можете отправить ему любую сумму денег на карту (Сбер):",
        ))
        .child(Panel::new(TextView::new(
            "2202 2062 5233 5406 (Михаил Сергеевич К.)",
        )))
        .child(TextView::new("Заранее спасибо!"));

    let win = Dialog::around(layout)
        .title("О программе")
        .button("OK", |s| {
            s.pop_layer();
        });
    scr.add_layer(win);
}

pub fn is_exit_win(scr: &mut Cursive) {
    let text = TextView::new("Действительно выйти?");
    let win = Dialog::around(text)
        .title("Выход")
        .button("Нет", |s| {
            s.pop_layer();
        })
        .button("Да", |s| s.quit());
    scr.add_layer(win);
}
