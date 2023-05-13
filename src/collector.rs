use crate::tables;
use gtk::traits::{BoxExt, ButtonExt, EditableExt, GtkWindowExt};
use polars_lazy::frame::LazyFrame;

pub(crate) struct DataCollector {
    sql_context: polars_sql::SQLContext,
    query: gtk::Text,
    button: gtk::Button,
}

impl DataCollector {
    pub(crate) fn from_lazy_frame(lazy_frame: LazyFrame) -> DataCollector {
        let mut sql_context = polars_sql::SQLContext::try_new().unwrap();
        sql_context.register("data", lazy_frame);

        let query = gtk::Text::builder()
            .text("select * from data where true limit 30")
            .build();

        let button = gtk::Button::builder()
            .label("Update")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

        DataCollector { sql_context, query, button }
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

        let table = self.table();
        vert_box.append(&table);

        let sql_context = self.sql_context.clone();
        let query = self.query.clone();
        self.button.connect_clicked(move |_| {
            let query = query.text().as_str().to_owned();
            let df = sql_context.clone().execute(&query).unwrap().collect().unwrap();
            let grid = tables::grid_from_frame(&df);
            let table = table_from_grid(&grid);
            vert_box.append(&table);
        });

        window
    }

    fn table(&mut self) -> gtk::ScrolledWindow {
        let query = self.query.text().as_str().to_owned();
        let df = self.sql_context.execute(&query).unwrap().collect().unwrap();
        let grid = tables::grid_from_frame(&df);
        table_from_grid(&grid)
    }
}

fn table_from_grid(grid: &gtk::Grid) -> gtk::ScrolledWindow {
    let table = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    table.set_child(Some(grid));
    table
}