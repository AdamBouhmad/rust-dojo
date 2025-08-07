use chrono::{Local, TimeZone, Duration};

pub fn local_day_bounds(year: i32, month: u32, day: u32) -> (i64, i64) {
    // Start of the day at 00:00 local time
    let start = Local.ymd(year, month, day).and_hms(0, 0, 0);
    // End of the day at 23:59:59 local time
    let end = start + Duration::days(1) - Duration::seconds(1);
    (start.timestamp(), end.timestamp())
}