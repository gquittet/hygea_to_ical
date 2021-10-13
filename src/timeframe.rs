use chrono::NaiveDate;

#[derive(Debug)]
pub struct Timeframe {
    pub start: i64,
    pub end: i64,
}

pub fn generate_timeframe(year: i32) -> Timeframe {
    let start = NaiveDate::from_ymd(year, 1, 1).and_hms(0, 0, 0).timestamp();
    let end = NaiveDate::from_ymd(year + 1, 1, 1)
        .and_hms(0, 0, 0)
        .timestamp();

    Timeframe { start, end }
}
