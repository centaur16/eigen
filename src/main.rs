use cursive::views::{Dialog, TextView};
use cursive::event::Key;
use cursive::theme::Color;

fn main() {
    let mut win = cursive::default();

    win.update_theme(
        |t| {
                t.shadow = false;
                t.palette.set_color("Background", Color::Rgb(10, 10, 10));
                t.palette.set_color("View", Color::Rgb(10, 10, 10));
                t.palette.set_color("Primary", Color::Rgb(255, 255, 255));
            });

    win.add_global_callback(Key::Esc, |w| w.quit());
    win.add_layer(TextView::new("Hello, world!"));
    win.run();
}
