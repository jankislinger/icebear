use crate::tables;
use gtk::traits::{BoxExt, ButtonExt, GtkWindowExt, TextViewExt};
use polars_lazy::frame::LazyFrame;
use gtk::prelude::TextBufferExt;

pub(crate) struct DataCollector {
    sql_context: polars_sql::SQLContext,
    query: gtk::TextView,
    button: gtk::Button,
    table_wrapper: gtk::ScrolledWindow,
}

impl DataCollector {
    pub(crate) fn from_lazy_frame(lazy_frame: LazyFrame) -> DataCollector {
        let mut sql_context = polars_sql::SQLContext::try_new().unwrap();
        sql_context.register("data", lazy_frame);

        let query = gtk::TextView::builder()
            .editable(true)
            .monospace(true)
            .wrap_mode(gtk::WrapMode::Word)
            .height_request(200)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(12)
            .margin_end(12)
            .build();

        query.buffer().set_text("select *\nfrom data\nwhere true\nlimit 150");


        let button = gtk::Button::builder()
            .label("Update")
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(12)
            .margin_end(12)
            .build();

        let table_wrapper = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .height_request(600)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(12)
            .margin_end(12)
            .build();

        DataCollector { sql_context, query, button, table_wrapper }
    }

    pub(crate) fn make_window(&mut self, app: &gtk::Application, subtitle: &str) -> gtk::ApplicationWindow {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title(format!("Polars Viewer: {}", subtitle))
            .default_width(800)
            .default_height(600)
            .build();

        let vert_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        window.set_child(Some(&vert_box));
        vert_box.append(&self.query);
        vert_box.append(&self.button);
        vert_box.append(&self.table_wrapper);

        // TODO: do we need to clone here?
        let sql_context = self.sql_context.clone();
        let query = self.query.clone();
        let table_wrapper = self.table_wrapper.clone();

        self.button.connect_clicked(move |_| {
            // TODO: can I call method fill_table here?
            let (start, end) = query.buffer().bounds();
            let query = query.buffer().text(&start, &end, true);
            let df = sql_context.clone().execute(&query).unwrap().collect().unwrap();
            let grid = tables::grid_from_frame(&df);
            table_wrapper.set_child(Some(&grid));
        });

        self.fill_table();
        window
    }

    fn fill_table(&mut self) {
        let (start, end) = self.query.buffer().bounds();
        let query = self.query.buffer().text(&start, &end, true);
        let df = self.sql_context.clone().execute(&query)
            .expect("SQL query failed")
            .collect()
            .expect("Query cannot be collected");
        let grid = tables::grid_from_frame(&df);
        self.table_wrapper.set_child(Some(&grid));
    }
}
