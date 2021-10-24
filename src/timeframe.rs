use chrono::NaiveDate;

#[derive(Debug, PartialEq)]
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

#[test]
fn it_generate_correct_timeframe_with_positive_date() {
    let expected = Timeframe {
        start: 1609459200, // 2021-01-01 00:00:00 UTC
        end: 1640995200,   // 2022-01-01 00:00:00 UTC
    };

    let result = generate_timeframe(2021);

    assert_eq!(expected, result)
}
