
use gtk::{Align, Grid, Label};
use polars::frame::DataFrame;
use gtk::prelude::*;
use polars::io::SerReader;
use polars::prelude::*;

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


#[cfg(test)]
mod test {
    use polars::frame::row::Row;
    use polars::prelude::AnyValue;
    use super::*;

    #[test]
    fn test_init() {
        gtk::init().unwrap();

        let df = DataFrame::from_rows(&vec![
            Row::new(vec![
                AnyValue::from(1),
                AnyValue::from(3),
                AnyValue::from(3),
            ]),
        ]).unwrap();
        let _ = grid_from_frame(&df);
    }
}
