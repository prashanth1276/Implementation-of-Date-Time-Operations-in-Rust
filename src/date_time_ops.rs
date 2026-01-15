#![allow(dead_code)]
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};

/// 181. Add Months
pub fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month() as i32 + months;

    year += (month - 1) / 12;
    month = (month - 1) % 12 + 1;

    let last_day = days_in_month(year, month as u32);
    let day = date.day().min(last_day);

    NaiveDate::from_ymd_opt(year, month as u32, day).unwrap()
}

/// 182. Add Years
pub fn add_years(date: NaiveDate, years: i32) -> NaiveDate {
    let year = date.year() + years;
    let last_day = days_in_month(year, date.month());
    let day = date.day().min(last_day);

    NaiveDate::from_ymd_opt(year, date.month(), day).unwrap()
}

/// 183. Add Hours
pub fn add_hours(date: NaiveDateTime, hours: i64) -> NaiveDateTime {
    date + Duration::hours(hours)
}

/// 184. Add Minutes
pub fn add_minutes(date: NaiveDateTime, minutes: i64) -> NaiveDateTime {
    date + Duration::minutes(minutes)
}

/// 185. Add Seconds
pub fn add_seconds(date: NaiveDateTime, seconds: i64) -> NaiveDateTime {
    date + Duration::seconds(seconds)
}

/// 186. Subtract Days
pub fn subtract_days(date: NaiveDate, days: i64) -> NaiveDate {
    date - Duration::days(days)
}

/// 187. Difference in Hours
pub fn diff_in_hours(d1: NaiveDateTime, d2: NaiveDateTime) -> i64 {
    (d1 - d2).num_hours().abs()
}

/// 188. Difference in Minutes
pub fn diff_in_minutes(d1: NaiveDateTime, d2: NaiveDateTime) -> i64 {
    (d1 - d2).num_minutes().abs()
}

/// 189. Difference in Seconds
pub fn diff_in_seconds(d1: NaiveDateTime, d2: NaiveDateTime) -> i64 {
    (d1 - d2).num_seconds().abs()
}

/// 190. Difference in Months/*  */
pub fn diff_in_months(d1: NaiveDate, d2: NaiveDate) -> i32 {
    ((d1.year() - d2.year()) * 12 + (d1.month() as i32 - d2.month() as i32)).abs()
}

/// 191. Difference in Years
pub fn diff_in_years(d1: NaiveDate, d2: NaiveDate) -> i32 {
    (d1.year() - d2.year()).abs()
}

/// 192. Start of Month
pub fn start_of_month(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap()
}

/// 193. End of Month
pub fn end_of_month(date: NaiveDate) -> NaiveDateTime {
    let last_day = days_in_month(date.year(), date.month());
    NaiveDate::from_ymd_opt(date.year(), date.month(), last_day)
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap()
}

/// 194. Start of Year
pub fn start_of_year(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap()
}

/// 195. End of Year
pub fn end_of_year(date: NaiveDate) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(date.year(), 12, 31)
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap()
}

/// 196. Start of Week (Monday = 1)
pub fn start_of_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday() as i64;
    date - Duration::days(weekday)
}

/// 197. End of Week
pub fn end_of_week(date: NaiveDate) -> NaiveDateTime {
    start_of_week(date)
        .and_hms_opt(0, 0, 0)
        .unwrap()
        + Duration::days(6)
        + Duration::hours(23)
        + Duration::minutes(59)
        + Duration::seconds(59)
}

/// 198. Is Same Day
pub fn is_same_day(d1: NaiveDateTime, d2: NaiveDateTime) -> bool {
    d1.date() == d2.date()
}

/// 199. Is Same Month
pub fn is_same_month(d1: NaiveDate, d2: NaiveDate) -> bool {
    d1.year() == d2.year() && d1.month() == d2.month()
}

/// 200. Is Same Year
pub fn is_same_year(d1: NaiveDate, d2: NaiveDate) -> bool {
    d1.year() == d2.year()
}

/// Helper: Days in Month
fn days_in_month(year: i32, month: u32) -> u32 {
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };
    (next_month - Duration::days(1)).day()
}
