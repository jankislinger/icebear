use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{glib, Application};

mod collector;
mod tables;

const APP_ID: &str = "org.icebear.IceBearExplorer";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_open(|app, files, _| {
        if let Some(file) = files.first() {
            if let Some(path) = file.path() {
                build_ui(app, path.to_str().unwrap());
                return;
            }
        }

        eprintln!("No file provided. Usage: icebear <FILE>");
        app.quit();
    });

    app.run()
}

fn build_ui(app: &Application, file_name: &str) {
    let frame = tables::load_frame(&file_name);
    collector::DataCollector::from_lazy_frame(frame)
        .make_window(app, &file_name)
        .present();
}
