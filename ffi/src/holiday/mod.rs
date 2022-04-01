#[cfg(test)]
mod test;

use core::holidays::{self, error::Error, Easter};
use std::os::raw::c_char;

use crate::{get_c_string, Date};

#[repr(C)]
#[derive(Debug)]
pub struct Holiday {
    pub name: *const c_char,
    pub date: Date,
}

#[repr(C)]
#[derive(Clone)]
pub struct HasHolidayResult {
    pub has_holiday: bool,
    pub holiday: *const Holiday,
}

#[repr(C)]
pub struct HolidayEaster(Easter);

#[no_mangle]
pub unsafe extern "C" fn holidays_get_easter(year: i32, date: *mut *mut HolidayEaster) -> bool {
    let easter = holidays::get_easter(year);

    match easter {
        Ok(easter) => {
            let date = &mut *date;
            let date_value = HolidayEaster(easter);
            **date = date_value;
            false
        }

        Err(_) => true,
    }
}

#[no_mangle]
pub unsafe extern "C" fn holiday_easter_into_date(easter: *const HolidayEaster) -> Date {
    let easter = &*easter;
    easter.0.get_date().into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_carnival(easter: *const HolidayEaster) -> Date {
    let easter = &*easter;
    holidays::get_carnival(&easter.0).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_corpus_christi(easter: *const HolidayEaster) -> Date {
    let easter = &*easter;
    holidays::get_corpus_christi(&easter.0).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_new_year(year: i32) -> Date {
    holidays::get_new_year(year).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_christmas(year: i32) -> Date {
    holidays::get_christmas(year).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_tiradentes(year: i32) -> Date {
    holidays::get_tiradentes(year).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_workers(year: i32) -> Date {
    holidays::get_workers(year).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_independence(year: i32) -> Date {
    holidays::get_independence(year).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_nossa_senhora_aparecida(year: i32) -> Date {
    holidays::get_nossa_senhora_aparecida(year).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_finados(year: i32) -> Date {
    holidays::get_finados(year).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_procration_of_the_republic(year: i32) -> Date {
    holidays::get_procration_of_the_republic(year).into()
}

#[no_mangle]
pub unsafe extern "C" fn holiday_get_all(year: i32, holidays: *mut Vec<Holiday>) -> bool {
    let holidays = &mut *holidays;
    let result = holidays::get_all_holidays(year);
    if let Ok(result) = result {
        let result_format = result
            .into_iter()
            .map(|(date, name)| Holiday {
                name: get_c_string(&name),
                date: date.into(),
            })
            .collect::<Vec<_>>();

        *holidays = result_format;

        false
    } else {
        true
    }
}

#[no_mangle]
pub unsafe extern "C" fn is_holiday(date: Date, holiday_result: *mut HasHolidayResult) -> bool {
    let is_holiday = holidays::is_holiday(date.clone().into());
    match is_holiday {
        Err(Error::InvalidYear) => true,
        Err(Error::NotHoliday) => {
            let holiday_result = &mut *holiday_result;
            *holiday_result = HasHolidayResult {
                has_holiday: false,
                holiday: std::ptr::null(),
            };
            false
        }
        Ok(holiday_name) => {
            let holiday_result = &mut *holiday_result;
            *holiday_result = HasHolidayResult {
                has_holiday: true,
                holiday: Box::into_raw(Box::new(Holiday {
                    name: get_c_string(&holiday_name),
                    date,
                })),
            };
            false
        }
    }
}
