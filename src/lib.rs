extern crate chrono;

use chrono::NaiveDateTime;

fn epoch2time(x: i64, d: i64, s: i64) -> NaiveDateTime {
    let q = x / d;
    let n = ((x % d) * (1_000_000_000/d)) as u32;
    NaiveDateTime::from_timestamp(q + s, n)
}

pub fn chrome (num: i64) -> NaiveDateTime {
	epoch2time(num, 1_000_000, -11_644_473_600)
}
pub fn cocoa (num: i64) -> NaiveDateTime {
	epoch2time(num, 1, 978_307_200)
}
pub fn java (num: i64) -> NaiveDateTime {
	epoch2time(num, 1000, 0)
}
pub fn mozilla (num: i64) -> NaiveDateTime {
	epoch2time(num, 1_000_000, 0)
}
pub fn symbian (num: i64) -> NaiveDateTime {
    epoch2time(num, 1_000_000, -62_167_219_200)
}
pub fn unix (num: i64) -> NaiveDateTime {
    epoch2time(num, 1, 0)
}
pub fn uuid_v1 (num: i64) -> NaiveDateTime {
	epoch2time(num, 10_000_000, -12_219_292_800)
}
pub fn windows_date (num: i64) -> NaiveDateTime {
    epoch2time(num, 10_000_000, -62_135_596_800)
}
pub fn windows_file (num: i64) -> NaiveDateTime {
    epoch2time(num, 10_000_000, -11_644_473_600)
}
    
#[cfg(test)]
mod tests {

    use super::{chrome, cocoa, java, mozilla, symbian, unix, uuid_v1,
    windows_date, windows_file};
    
    #[test]
    fn chrome_run() {
        let ndt = chrome(12879041490000000);
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }

    #[test]
    fn chrome_with_micros() {
        let ndt = chrome(12_912_187_816_559_001);
        assert_eq!(ndt.to_string(), "2010-03-04 14:50:16.559001");
    }

    #[test]
    fn cocoa_run() {
        let ndt = cocoa(256260690);
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }

    #[test]
    fn java_run() {
        let ndt = java(1234567890000);
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    
    #[test]
    fn mozilla_run() {
        let ndt = mozilla(1234567890000000);
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }

    #[test]
    fn symbian_run() {
        let ndt = symbian(63401787090000000);
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }

    #[test]
    fn unix_run() {
        let ndt = unix(1234567890);
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn unix_minus_run() {
        let ndt = unix(-1234567890);
        assert_eq!(ndt.to_string(), "1930-11-18 00:28:30");
    }

    #[test]
    fn uuid_run() {
        let ndt = uuid_v1(134538606900000000);
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn uuid_micros() {
        let ndt = uuid_v1(0x1dc7711a73088f5);
        assert_eq!(ndt.to_string(), "2007-10-10 09:17:41.739749300");
    }

    #[test]
    fn windows_date_run() {
        let ndt = windows_date(633701646900000000);
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn windows_date_micros() {
        let ndt = windows_date(634496538123456789);
        assert_eq!(ndt.to_string(), "2011-08-22 23:50:12.345678900");
    }

    #[test]
    fn windows_file_run() {
        let ndt = windows_file(128790414900000000);
        assert_eq!(ndt.to_string(), "2009-02-13 23:31:30");
    }
    #[test]
    fn windows_file_micros() {
        let ndt = windows_file(0x1cabbaa00ca9000);
        assert_eq!(ndt.to_string(), "2010-03-04 14:50:16.559001600");
    }
    
}

