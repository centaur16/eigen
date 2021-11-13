use cursive::views::{Dialog, TextView};
use cursive::event::Key;
use cursive::menu;
use cursive::theme::BaseColor::{Black, White};
use cursive::theme::Color;

fn main() {
    let mut win = cursive::default();

    win.update_theme(
        |t| {
                t.shadow = false;
                t.palette.set_color("Background", Color::Dark(Black));
                t.palette.set_color("View", Color::Dark(Black));
                t.palette.set_color("Primary", Color::Dark(White));
            });

    win.menubar()
        .add_subtree(
            "File",
            menu::MenuTree::new()
                .leaf("New", move |s| {
                    s.add_layer(Dialog::info("New file!"));
                })
        );

    win.set_autohide_menu(false);

    win.add_global_callback('q', |w| w.quit());
    win.add_global_callback(Key::Esc, |w| w.select_menubar());
    win.add_layer(TextView::new("Hello, world!"));
    win.run();
}
