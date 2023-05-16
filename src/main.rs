
use gtk::gio::{ApplicationFlags};
use gtk::prelude::*;
use gtk::{Application, glib};
use std::env;




mod tables;
mod collector;

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
    let frame = tables::load_frame(&file_name);
    collector::DataCollector::from_lazy_frame(frame)
        .make_window(app, &file_name)
        .present();
}
