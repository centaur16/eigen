use std::vec;
use cursive::views::{Dialog, LinearLayout, Panel, ScrollView, TextArea, TextView};
use cursive::event::Key;
use cursive::menu;
use cursive::theme::BaseColor::{Black, White};
use cursive::theme::Color;
use crate::menu::MenuTree;

struct Profile {
    identifier: String,
    description: String,
}

fn main() {
    let mut win = cursive::default();

    let mut profile_menu = MenuTree::new();
    (0..3)
        .map(|i| Profile { identifier: format!("profile{}", i), description: format!("Profile {}", i) })
        .for_each(|profile| {
            profile_menu.add_leaf(profile.description.clone(), move |s| {
                s.add_layer(Dialog::info(format!("Selected {}", profile.description.clone())))
            })
        });

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
                }),
        )
        .add_subtree(
            "Accounts",
            menu::MenuTree::new()
                .leaf("Add Account Profile", move |s| {
                    s.add_layer(Dialog::info("Adding a new Account profile"));
                })
                .delimiter()
                .subtree("Switch Account Profile", profile_menu),
        );

    win.set_autohide_menu(false);

    win.add_layer(LinearLayout::horizontal()
        .child(ScrollView::new(Panel::new(
            TextArea::new()
                .content("Hello World")
        )))
    );

    win.add_global_callback('q', |w| w.quit());
    win.add_global_callback(Key::Esc, |w| w.select_menubar());
    win.add_layer(TextView::new("Hello, world!"));
    win.run();
}
