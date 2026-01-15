mod date_time_ops;
use chrono::NaiveDate;

fn main() {
    let d = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
    println!("{:?}", date_time_ops::add_months(d, 3));
}
