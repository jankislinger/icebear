use std::cell::Cell;
use gtk::gio::{ApplicationFlags};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib, Orientation};
use std::env;
use polars::prelude::DataFrame;
use polars_lazy::frame::{LazyFrame, ScanArgsParquet};
use polars_sql::SQLContext;

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
        .default_width(1000)
        .default_height(1000)
        .child(&get_layout(&file_name))
        .build()
        .present();
}

fn get_layout(file_name: &str) -> gtk::Box {
    let vert_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let query_box = text_input_query();
    vert_box.append(&query_box);
    vert_box.append(&submit_button(&file_name, &vert_box, &query_box));
    vert_box.append(&table_container(&file_name, &query_box));

    return vert_box;
}

fn table_container(file_name: &str, query_box: &gtk::Text) -> gtk::Grid {
    let query = query_box.text().as_str().to_owned();
    let df = tables::run_query(&file_name, &query);
    let grid = tables::grid_from_frame(&df);
    grid
}

fn text_input_query() -> gtk::Text {
    gtk::Text::builder()
        .text("select * from data where completion > 0.9")
        .build()
}

fn submit_button(file_name: &str, vbox: &gtk::Box, query_box: &gtk::Text) -> gtk::Button {
    let button = gtk::Button::builder()
        .label("Update")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let fname_copy = file_name.to_owned();
    button.connect_clicked(|a| {
        println!("Button clicked");
        vbox.append(&table_container(&fname_copy, &query_box))
    });
    return button;
}


struct Viewer {
    context: Cell<SQLContext>,
    query: gtk::Text,
}

impl Viewer {
    fn from_file(file_name: &str) -> Viewer {
        let args = ScanArgsParquet::default();
        let mut context = SQLContext::try_new().unwrap();
        let frame = LazyFrame::scan_parquet(&file_name, args)
            .expect(&format!("Cannot load parquet {}", &file_name));
        context.register("data", frame);

        let query = gtk::Text::builder()
            .text("select * from data where completion > 0.9")
            .build();

        Viewer { context: Cell::new(context), query }
    }

    fn get_data(&self) -> DataFrame {
        // TODO: return result
        self.context
            .get()
            .execute(&self.query.text())
            .unwrap()
            .limit(30)
            .collect()
            .unwrap()
    }
}
