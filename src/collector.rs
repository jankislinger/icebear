use crate::tables;
use gtk::traits::{BoxExt, ButtonExt, EditableExt, GtkWindowExt};
use polars_lazy::frame::LazyFrame;

pub(crate) struct DataCollector {
    sql_context: polars_sql::SQLContext,
    query: gtk::Text,
    button: gtk::Button,
    table_wrapper: gtk::ScrolledWindow,
}

impl DataCollector {
    pub(crate) fn from_lazy_frame(lazy_frame: LazyFrame) -> DataCollector {
        let mut sql_context = polars_sql::SQLContext::try_new().unwrap();
        sql_context.register("data", lazy_frame);

        let query = gtk::Text::builder()
            .text("select * from data where true limit 150")
            .build();

        let button = gtk::Button::builder()
            .label("Update")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        let table_wrapper = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .height_request(600)
            .build();

        DataCollector { sql_context, query, button, table_wrapper }
    }

    pub(crate) fn make_window(&mut self, app: &gtk::Application) -> gtk::ApplicationWindow {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Polars Viewer")
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
            let query = query.text().as_str().to_owned();
            let df = sql_context.clone().execute(&query).unwrap().collect().unwrap();
            let grid = tables::grid_from_frame(&df);
            table_wrapper.set_child(Some(&grid));
        });

        self.fill_table();
        window
    }

    fn fill_table(&mut self) {
        let query = self.query.text().as_str().to_owned();
        let df = self.sql_context.clone().execute(&query).unwrap().collect().unwrap();
        let grid = tables::grid_from_frame(&df);
        self.table_wrapper.set_child(Some(&grid));
    }
}
