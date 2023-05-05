use gtk::{Align, Grid, Label, traits::GridExt};
use polars::frame::DataFrame;
use polars_lazy::frame::{LazyFrame, ScanArgsParquet};
use polars_sql::SQLContext;

pub fn grid_from_frame(df: &DataFrame) -> Grid {
    let grid = Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(false)
        .row_spacing(5)
        .column_spacing(25)
        .halign(Align::Start)
        .build();

    for (i, col) in df.iter().into_iter().enumerate() {
        col.name();

        let header = Label::new(Some(col.name()));
        grid.attach(&header, i as i32, 0, 1, 1);

        for (j, x) in col.iter().into_iter().enumerate() {
            let j = (j as i32) + 1;
            let label = Label::new(Some(&format!("{}", x)));
            grid.attach(&label, i as i32, j, 1, 1);
        }
    }
    grid
}

pub(crate) fn load_frame(file_name: &str) -> DataFrame {
    let args = ScanArgsParquet::default();
    LazyFrame::scan_parquet(&file_name, args)
        .expect(&format!("Cannot load parquet {}", &file_name))
        .limit(30)
        .collect().unwrap()
}


pub(crate) fn run_query(file_name: &str, query: &str) -> DataFrame {
    let args = ScanArgsParquet::default();
    let lazy = LazyFrame::scan_parquet(&file_name, args)
        .expect(&format!("Cannot load parquet {}", &file_name));
    let mut ctx = SQLContext::try_new().unwrap();
    ctx.register("data", lazy);
    ctx.execute(&query)
        .unwrap()
        .limit(25)
        .collect()
        .unwrap()
}


#[cfg(test)]
mod test {
    use polars::df;
    use polars::prelude::NamedFrom;
    use polars_lazy::prelude::*;
    use polars_sql::SQLContext;
    use super::*;

    #[test]
    fn test_init() {
        gtk::init().unwrap();

        let df = df! {
            "x" => &[1, 2, 3],
        }.unwrap();
        let _ = grid_from_frame(&df);
    }

    #[test]
    fn test_doc() {
        let mut ctx = SQLContext::try_new().unwrap();
        let df = df! {
           "a" =>  &[1, 2, 3],
        }.unwrap();

        let a = LazyFrame::try_from(df.clone().lazy());

        ctx.register("df", a.unwrap());
        let sql_df = ctx.execute("SELECT * FROM df").unwrap().collect().unwrap();
        assert!(sql_df.frame_equal(&df));
    }
}
