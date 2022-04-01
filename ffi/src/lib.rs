#![allow(clippy::missing_safety_doc)]

use core::prelude::{Date as CDate, Datelike, TimeZone, Utc};
use std::{ffi::CString, os::raw::c_char};

pub mod holiday;

pub fn get_c_string(s: &str) -> *const c_char {
    CString::new(s).unwrap().into_raw()
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl From<CDate<Utc>> for Date {
    fn from(date: CDate<Utc>) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }
}

impl From<Date> for CDate<Utc> {
    fn from(date: Date) -> Self {
        Utc.ymd(date.year, date.month, date.day)
    }
}
