use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};
use polars::io::SerReader;
use std::cell::Cell;
use std::env;
use std::rc::Rc;

mod tables;

const APP_ID: &str = "org.dfgui.HelloWorld";
const DEFAULT: &str = "data.parquet";


fn main() -> glib::ExitCode {

    // let args: Vec<String> = env::args().collect();
    // let file_name = args[1].clone();
    let file_name = env::var("FILE_NAME")
        .unwrap_or(String::from("data.parquet"));
    // I wasn't able to make `cargo run -- <other args>` work, no idea


    dbg!(&file_name);
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(move |a| build_ui(a, &file_name));
    app.run()
}


fn build_ui(app: &Application, file_name: &str) {
    ApplicationWindow::builder()
        .application(app)
        .title("Data Frame")
        .default_width(350)
        .default_height(250)
        .child(&table_container(&file_name))
        .build()
        .present()
}

fn table_container(file_name: &str) -> gtk::Grid {
    let df = tables::load_frame(&file_name);
    let grid = tables::grid_from_frame(&df);
    grid
}

fn button_counter(cnt: &Rc<Cell<i32>>) -> gtk::Button {
    let button = gtk::Button::builder()
        .label("Some button")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    button.connect_clicked(clone!(@strong cnt => move |b| {
        cnt.set(cnt.get() + 1);
        b.set_label(&format!("Clicked {} times!", cnt.get()));
    }));
    return button;
}
