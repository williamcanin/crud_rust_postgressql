use chrono::NaiveDate;

pub fn set_date(year: i32, month: u32, day: u32) -> NaiveDate {
  NaiveDate::from_ymd_opt(year, month, day)
    .unwrap_or_else(|| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap())
}
