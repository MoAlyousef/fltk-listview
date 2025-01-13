use fltk::{prelude::*, *};
use fltk_listview::{ListView, ListViewContext};

const HEADERS: [&str; 4] = ["Name", "Breed", "Gender", "Price"];
const DATA: [[&str; 4]; 5] = [
    ["Oscar", "Siamese", "Male", "$175.00"],
    ["Molly", "Poodle", "Female", "$300.00"],
    ["Brutus", "Manx", "Male", "$250.00"],
    ["Portia", "Mastiff", "Female", "$500.00"],
    ["Pompey", "Mixed", "Male", "$80.00"],
];

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default()
        .with_size(600, 400)
        .with_label("FLTK ListView Example");
    let mut listview = ListView::<4, 5>::default().with_size(400, 300).center_of_parent();
    listview.set_data(HEADERS, DATA);
    listview.set_callback(|l, ctx| {
        match ctx {
            ListViewContext::Cell => println!(
                "row: {}, column: {}, {}: {}",
                l.callback_row(),
                l.callback_col(),
                HEADERS[l.callback_col() as usize],
                DATA[l.callback_row() as usize][l.callback_col() as usize]
            ),
            ListViewContext::ColHeader => {
                println!("header: {}", HEADERS[l.callback_col() as usize])
            }
            _ => (),
        };
    });
    wind.resizable(&*listview);
    wind.end();
    wind.show();
    app.run().unwrap();
}
