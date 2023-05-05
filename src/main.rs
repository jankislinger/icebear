use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};
use std::env;
use gtk::gio::{ApplicationFlags};

mod tables;

const APP_ID: &str = "org.dfgui.HelloWorld";


fn main() -> glib::ExitCode {

    let args: Vec<String> = env::args().collect();
    let file_name = args[1].clone();


    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_open(move |a, _, _| {
        build_ui(a, &file_name);
    });

    app.run()
}

fn build_ui(app: &Application, file_name: &str) {
    ApplicationWindow::builder()
        .application(app)
        .title(format!("Data Frame: {}", &file_name))
        .default_width(350)
        .default_height(250)
        .child(&table_container(&file_name))
        .build()
        .present();
}

fn table_container(file_name: &str) -> gtk::Grid {
    let df = tables::load_frame(&file_name);
    let grid = tables::grid_from_frame(&df);
    grid
}
