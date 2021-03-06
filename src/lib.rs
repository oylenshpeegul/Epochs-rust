//! Convert various epoch times to [chrono::NaiveDateTime](https://docs.rs/chrono/0.4.10/chrono/naive/struct.NaiveDateTime.html) times.

extern crate chrono;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};

// The icq function uses time::Duration, which panics if given too big
// a number. The maximum is i64::MAX milliseconds.
const MAX_DAYS: i64 = std::i64::MAX / (24 * 60 * 60 * 1000);

const MILLIS_PER_DAY: f64 = 24. * 60. * 60. * 1000.;

/// APFS time is the number of nanoseconds since the Unix epoch
/// (*cf.*, [APFS filesystem format](https://blog.cugu.eu/post/apfs/)).
///
/// ```
/// use epochs::apfs;
/// let ndt = apfs(1_234_567_890_000_000_000).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn apfs(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 1_000_000_000, 0)
}

/// Convert the given NaiveDateTime to an [APFS](fn.apfs.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_apfs;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_apfs(ndt), 1_234_567_890_000_000_000);
/// ```
pub fn to_apfs(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 1_000_000_000, 0)
}

/// Chrome time is the number of microseconds since 1601-01-01, which
/// is 11,644,473,600 seconds before the Unix epoch.
///
/// ```
/// use epochs::chrome;
/// let ndt = chrome(12_879_041_490_000_000).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn chrome(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 1_000_000, -11_644_473_600)
}

/// Convert the given NaiveDateTime to a [Chrome](fn.chrome.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_chrome;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_chrome(ndt), 12_879_041_490_000_000);
/// ```
pub fn to_chrome(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 1_000_000, -11_644_473_600)
}

/// Cocoa time is the number of seconds since 2001-01-01, which is
/// 978,307,200 seconds after the Unix epoch.
///
/// ```
/// use epochs::cocoa;
/// let ndt = cocoa(256260690).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn cocoa(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 1, 978_307_200)
}

/// Convert the given NaiveDateTime to a [Cocoa](fn.cocoa.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_cocoa;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_cocoa(ndt), 256260690);
/// ```
pub fn to_cocoa(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 1, 978_307_200)
}

/// Google Calendar time seems to count 32-day months from the day
/// before the Unix epoch ([@noppers](https://github.com/noppers)
/// worked out how to do this).
///
/// ```
/// use epochs::google_calendar;
/// let ndt = google_calendar(1297899090).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn google_calendar(num: i64) -> Option<NaiveDateTime> {
    let seconds_per_day = 24 * 60 * 60;
    let total_days = num / seconds_per_day;
    let seconds = num % seconds_per_day;

    let months = total_days / 32;
    let days = total_days % 32;

    // The Google epoch starts a day early.
    let ndt = NaiveDate::from_ymd(1969, 12, 31).and_hms(0, 0, 0);

    // First, add the days...
    let ndt = ndt + Duration::days(days);

    // ...then the months...
    let ndt = plus_months(ndt, months)?;

    // ...then the seconds...
    let ndt = ndt + Duration::seconds(seconds);

    Some(ndt)
}

/// Convert the given NaiveDateTime to a [Google
/// Calendar](fn.google_calendar.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_google_calendar;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_google_calendar(ndt), 1297899090);
/// ```
pub fn to_google_calendar(ndt: NaiveDateTime) -> i64 {
    (((((ndt.year() as i64 - 1970) * 12 + (ndt.month() as i64 - 1)) * 32 + ndt.day() as i64) * 24
        + ndt.hour() as i64)
        * 60
        + ndt.minute() as i64)
        * 60
        + ndt.second() as i64
}

/// ICQ time is the number of days since 1899-12-30. Days can have a
/// fractional part.
///
/// ```
/// use epochs::icq;
/// let ndt = icq(39857.980208333334).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn icq(days: f64) -> Option<NaiveDateTime> {
    let intdays = days as i64;
    if intdays > MAX_DAYS {
        return None;
    }

    let milliseconds = ((days - (intdays as f64)) * MILLIS_PER_DAY) as i64;

    NaiveDate::from_ymd(1899, 12, 30)
        .and_hms(0, 0, 0)
        .checked_add_signed(Duration::days(intdays))?
        .checked_add_signed(Duration::milliseconds(milliseconds))
}

/// Convert the given NaiveDateTime to an [ICQ](fn.icq.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_icq;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_icq(ndt), 39857.980208333334);
/// ```
pub fn to_icq(ndt: NaiveDateTime) -> f64 {
    let diff = ndt - NaiveDate::from_ymd(1899, 12, 30).and_hms(0, 0, 0);
    diff.num_milliseconds() as f64 / MILLIS_PER_DAY
}

/// Java time is the number of milliseconds since the Unix epoch.
///
/// ```
/// use epochs::java;
/// let ndt = java(1_234_567_890_000).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn java(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 1000, 0)
}

/// Convert the given NaiveDateTime to a [Java](fn.java.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_java;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_java(ndt), 1_234_567_890_000);
/// ```
pub fn to_java(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 1000, 0)
}

/// Mozilla time (*e.g.*, Firefox) is the number of microseconds since
/// the Unix epoch.
///
/// ```
/// use epochs::mozilla;
/// let ndt = mozilla(1_234_567_890_000_000).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn mozilla(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 1_000_000, 0)
}

/// Convert the given NaiveDateTime to a [Mozilla](fn.mozilla.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_mozilla;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_mozilla(ndt), 1_234_567_890_000_000);
/// ```
pub fn to_mozilla(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 1_000_000, 0)
}

/// Symbian time is the number of microseconds since the year 0, which
/// is 62,167,219,200 seconds before the Unix epoch.
///
/// ```
/// use epochs::symbian;
/// let ndt = symbian(63_401_787_090_000_000).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn symbian(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 1_000_000, -62_167_219_200)
}

/// Convert the given NaiveDateTime to a [Symbian](fn.symbian.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_symbian;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_symbian(ndt), 63_401_787_090_000_000);
/// ```
pub fn to_symbian(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 1_000_000, -62_167_219_200)
}

/// Unix time is the number of seconds since 1970-01-01.
///
/// ```
/// use epochs::unix;
/// let ndt = unix(1234567890).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn unix(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 1, 0)
}

/// Convert the given NaiveDateTime to a [Unix](fn.unix.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_unix;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_unix(ndt), 1234567890);
/// ```
pub fn to_unix(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 1, 0)
}

/// UUID version 1 time ([RFC
/// 4122](https://tools.ietf.org/html/rfc4122)) is the number of
/// hectonanoseconds (100 ns) since 1582-10-15, which is
/// 12,219,292,800 seconds before the Unix epoch.
///
/// ```
/// use epochs::uuid_v1;
/// let ndt = uuid_v1(134_538_606_900_000_000).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
/// 

/// UUIDs typically appear in "8-4-4-4-12" strings like
/// 
/// &nbsp;&nbsp;&nbsp;&nbsp; ca4892ce-4f7d-11ea-b77f-2e728ce88125
/// 
/// where the timestamp portion is buried inside. This one is
/// "2020-02-14 23:00:27.148155". That first 1,
/// 
/// &nbsp;&nbsp;&nbsp;&nbsp; ca4892ce-4f7d-**1**1ea-b77f-2e728ce88125
/// 
/// means it's a version 1 UUID (other versions don't have timestamps
/// in them), so it's appropriate to take these bytes,
/// 
/// &nbsp;&nbsp;&nbsp;&nbsp; **ca4892ce**-**4f7d**-1**1ea**-b77f-2e728ce88125
/// 
/// make an integer, 0x1ea4f7dca4892ce, and
/// perform the calculation in this module on it.
/// 
/// ```
/// use epochs::uuid_v1;
/// let ndt = uuid_v1(0x1ea4f7dca4892ce).unwrap();
/// assert_eq!(ndt.to_string(), "2020-02-14 23:00:27.148155");
/// ```
pub fn uuid_v1(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 10_000_000, -12_219_292_800)
}

/// Convert the given NaiveDateTime to a [UUIDv1](fn.uuid_v1.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_uuid_v1;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_uuid_v1(ndt), 134_538_606_900_000_000);
/// ```
pub fn to_uuid_v1(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 10_000_000, -12_219_292_800)
}

/// Windows date time (e.g., .NET) is the number of hectonanoseconds
/// (100 ns) since 0001-01-01, which is 62,135,596,800 seconds before
/// the Unix epoch.
///
/// ```
/// use epochs::windows_date;
/// let ndt = windows_date(633_701_646_900_000_000).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn windows_date(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 10_000_000, -62_135_596_800)
}

/// Convert the given NaiveDateTime to a [Windows
/// Date](fn.windows_date.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_windows_date;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_windows_date(ndt), 633_701_646_900_000_000);
/// ```
pub fn to_windows_date(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 10_000_000, -62_135_596_800)
}

/// Windows file time (e.g., NTFS) is the number of hectonanoseconds
/// (100 ns) since 1601-01-01, which is 11,644,473,600 seconds before
/// the Unix epoch.
///
/// ```
/// use epochs::windows_file;
/// let ndt = windows_file(128_790_414_900_000_000).unwrap();
/// assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
/// ```
pub fn windows_file(num: i64) -> Option<NaiveDateTime> {
    epoch2time(num, 10_000_000, -11_644_473_600)
}

/// Convert the given NaiveDateTime to a [Windows
/// File](fn.windows_file.html) time.
///
/// ```
///# extern crate chrono;
/// use chrono::NaiveDateTime;
/// use epochs::to_windows_file;
/// let ndt = NaiveDateTime::parse_from_str("2009-02-13 23:31:30", "%Y-%m-%d %H:%M:%S").unwrap();
/// assert_eq!(to_windows_file(ndt), 128_790_414_900_000_000);
/// ```
pub fn to_windows_file(ndt: NaiveDateTime) -> i64 {
    time2epoch(ndt, 10_000_000, -11_644_473_600)
}

/// epoch2time adjusts the given epoch x by the given dividend d and
/// shift s and returns the result as a chrono::NaiveDateTime.
fn epoch2time(x: i64, d: i64, s: i64) -> Option<NaiveDateTime> {
    let q = x / d;
    let n = ((x % d) * (1_000_000_000 / d)) as u32;
    let t = q.checked_add(s)?;
    NaiveDateTime::from_timestamp_opt(t, n)
}

/// time2epoch adjusts the given chrono::NaiveDateTime ndt by the
/// multiplier m and the shift s and returns the result as a 64-bit
/// integer.
fn time2epoch(ndt: NaiveDateTime, m: i64, s: i64) -> i64 {
    let n = ndt.timestamp_subsec_nanos() as f64;
    let q = n / 1_000_000_000.0;
    let t = ndt.timestamp() as f64;
    let sf = s as f64;
    let mf = m as f64;
    (mf * (t + q - sf)) as i64
}

/// This function appears in the chrono documentation, but is not
/// actually provided as part of the package.
///
/// https://lifthrasiir.github.io/rust-chrono/chrono/naive/date/struct.NaiveDate.html#method.day
///
/// Combined with NaiveDate::pred, one can determine the number of
/// days in a particular month.
fn ndays_in_month(year: i32, month: u32) -> Option<i64> {
    // the first day of the next month...
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let d = NaiveDate::from_ymd_opt(y, m, 1)?;

    // ...is preceded by the last day of the original month
    Some(d.pred().day() as i64)
}

/// Add a month to the given NaiveDateTime by finding out how many
/// days are in the current month and adding that many days.
fn plus_month(ndt: NaiveDateTime) -> Option<NaiveDateTime> {
    let days = ndays_in_month(ndt.year(), ndt.month())?;
    Some(ndt + Duration::days(days))
}

/// Add the given number of months to the given NaiveDateTime.
fn plus_months(ndt: NaiveDateTime, months: i64) -> Option<NaiveDateTime> {
    let years = (months / 12) as i32;
    let months = months % 12;

    let mut ndt = ndt.with_year(ndt.year() + years)?;

    for _i in 0..months {
        ndt = plus_month(ndt)?;
    }
    Some(ndt)
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn apfs_run() {
        let ndt = apfs(1234567890000000000).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn to_apfs_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_apfs(ndt), 1234567890000000000);
    }

    #[test]
    fn chrome_run() {
        let ndt = chrome(12879041490000000).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn chrome_with_micros() {
        let ndt = chrome(12_912_187_816_559_001).unwrap();
        assert_eq!(ndt.to_string(), "2010-03-04 14:50:16.559001");
    }
    #[test]
    fn to_chrome_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_chrome(ndt), 12879041490000000);
    }

    #[test]
    fn cocoa_run() {
        let ndt = cocoa(256260690).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn to_cocoa_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_cocoa(ndt), 256260690);
    }

    #[test]
    fn google_calendar_run() {
        let ndt = google_calendar(1297899090).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn google_calendar_too_big() {
        let obs = google_calendar(12978990900000);
        assert_eq!(obs.is_none(), true);
    }
    #[test]
    fn to_google_calendar_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_google_calendar(ndt), 1297899090);
    }

    #[test]
    fn icq_run() {
        let ndt = icq(39857.980209).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30.057");
    }
    #[test]
    fn icq_too_big() {
        let obs = icq(398570000.980209);
        assert_eq!(obs.is_none(), true);
    }
    #[test]
    fn icq_way_too_big() {
        let obs = icq(123456789012.0);
        assert_eq!(obs.is_none(), true);
    }
    #[test]
    fn icq_frac() {
        let ndt = icq(41056.275208).unwrap();
        assert_eq!(ndt.to_string(), "2012-05-27 06:36:17.971");
    }
    #[test]
    fn to_icq_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert!(to_icq(ndt) - 39857.980209 < 1e-6);
    }
    #[test]
    fn to_icq_frac() {
        let ndt = NaiveDate::from_ymd(2012, 5, 27).and_hms_milli(6, 36, 17, 971);
        assert!(to_icq(ndt) - 41056.275208 < 1e-6);
    }

    #[test]
    fn java_run() {
        let ndt = java(1234567890000).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn to_java_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_java(ndt), 1234567890000);
    }

    #[test]
    fn mozilla_run() {
        let ndt = mozilla(1234567890000000).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn to_mozilla_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_mozilla(ndt), 1234567890000000);
    }

    #[test]
    fn symbian_run() {
        let ndt = symbian(63401787090000000).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn to_symbian_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_symbian(ndt), 63401787090000000);
    }

    #[test]
    fn unix_run() {
        let ndt = unix(1234567890).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn unix_minus_run() {
        let ndt = unix(-1234567890).unwrap();
        assert_eq!(ndt.to_string(), "1930-11-18 00:28:30");
    }
    #[test]
    fn to_unix_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_unix(ndt), 1234567890);
    }

    #[test]
    fn uuid_run() {
        let ndt = uuid_v1(134538606900000000).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn uuid_micros() {
        let ndt = uuid_v1(0x1dc7711a73088f5).unwrap();
        assert_eq!(ndt.to_string(), "2007-10-10 09:17:41.739749300");
    }
    #[test]
    fn to_uuid_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_uuid_v1(ndt), 134538606900000000);
    }

    #[test]
    fn windows_date_run() {
        let ndt = windows_date(633701646900000000).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn windows_date_micros() {
        let ndt = windows_date(634496538123456789).unwrap();
        assert_eq!(ndt.to_string(), "2011-08-22 23:50:12.345678900");
    }
    #[test]
    fn to_windows_date_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_windows_date(ndt), 633701646900000000);
    }

    #[test]
    fn windows_file_run() {
        let ndt = windows_file(128790414900000000).unwrap();
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn windows_file_micros() {
        let ndt = windows_file(0x1cabbaa00ca9000).unwrap();
        assert_eq!(ndt.to_string(), "2010-03-04 14:50:16.559001600");
    }
    #[test]
    fn to_windows_file_run() {
        let ndt = NaiveDate::from_ymd(2009, 2, 13).and_hms(23, 31, 30);
        assert_eq!(to_windows_file(ndt), 128790414900000000);
    }
}
