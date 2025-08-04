use chrono::{Months, Utc};
use query_args::{time_range::TimeRange};

fn main() {
    dotenvy::dotenv().ok();

    let end = Utc::now();
    let start = end.checked_sub_months(Months::new(1)).unwrap();
    let mut t = TimeRange::new(start, 5, end);
    while let Some(t) =  t.next() {
        println!("{} - {}", t.fmt_start().unwrap(), t.fmt_end().unwrap())
    }
}
