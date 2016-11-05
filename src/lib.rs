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

#[cfg(test)]
mod tests {
    use super::{chrome, cocoa, java, mozilla};
    
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
}

