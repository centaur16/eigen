use cursive::views::TextView;
use cursive::event::Key;

fn main() {
    let mut win = cursive::default();

    win.add_global_callback(Key::Esc, |w| w.quit());
    win.add_layer(TextView::new("Hello, world!"));
    win.run();
}
