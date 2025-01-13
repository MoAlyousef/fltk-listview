use fltk::{prelude::*, *};
use fltk_listview::ListView;

const HEADERS: &[&str] = &["Name", "Breed", "Gender", "Price\t"];
const DATA: &[&[&str]] = &[
    &["Oscar", "Siamese", "Male", "$175.00"],
    &["Molly", "Poodle", "Female", "$300.00"],
    &["Brutus", "Manx", "Male", "$250.00"],
    &["Portia", "Mastiff", "Female", "$500.00"],
    &["Pompey", "Mixed", "Male", "$80.00"],
];

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default()
        .with_size(600, 400)
        .with_label("FLTK ListView Example");
    let mut listview = ListView::default_fill();
    listview.set_table(5, 4);
    listview.set_data(HEADERS, DATA);
    listview.set_callback(|l| {
        println!(
            "{}",
            DATA[l.callback_row() as usize][l.callback_col() as usize]
        );
    });
    wind.resizable(&*listview);
    wind.end();
    wind.show();
    app.run().unwrap();
}
