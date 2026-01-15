mod date_time_ops;
use chrono::{NaiveDate};

fn main() {
    let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
    let dt = date.and_hms_opt(10, 0, 0).unwrap();
    let other_date = NaiveDate::from_ymd_opt(2023, 1, 15).unwrap();
    let other_dt = other_date.and_hms_opt(10, 0, 0).unwrap();

    println!("181 Add months: {:?}", date_time_ops::add_months(date, 2));
    println!("182 Add years: {:?}", date_time_ops::add_years(date, 1));
    println!("183 Add hours: {:?}", date_time_ops::add_hours(dt, 5));
    println!("184 Add minutes: {:?}", date_time_ops::add_minutes(dt, 30));
    println!("185 Add seconds: {:?}", date_time_ops::add_seconds(dt, 45));
    println!("186 Subtract days: {:?}", date_time_ops::subtract_days(date, 10));
    println!("187 Diff hours: {}", date_time_ops::diff_in_hours(dt, other_dt));
    println!("188 Diff minutes: {}", date_time_ops::diff_in_minutes(dt, other_dt));
    println!("189 Diff seconds: {}", date_time_ops::diff_in_seconds(dt, other_dt));
    println!("190 Diff months: {}", date_time_ops::diff_in_months(date, other_date));
    println!("191 Diff years: {}", date_time_ops::diff_in_years(date, other_date));
    println!("192 Start of month: {:?}", date_time_ops::start_of_month(date));
    println!("193 End of month: {:?}", date_time_ops::end_of_month(date));
    println!("194 Start of year: {:?}", date_time_ops::start_of_year(date));
    println!("195 End of year: {:?}", date_time_ops::end_of_year(date));
    println!("196 Start of week: {:?}", date_time_ops::start_of_week(date));
    println!("197 End of week: {:?}", date_time_ops::end_of_week(date));
    println!("198 Is same day: {}", date_time_ops::is_same_day(dt, other_dt));
    println!("199 Is same month: {}", date_time_ops::is_same_month(date, other_date));
    println!("200 Is same year: {}", date_time_ops::is_same_year(date, other_date));
}
