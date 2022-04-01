use crate::Date;

use super::HasHolidayResult;

#[test]
fn test_get_christmas() {
    unsafe {
        let date = super::holiday_get_christmas(2019);
        assert_eq!(
            date,
            Date {
                year: 2019,
                month: 12,
                day: 25,
            }
        );
    }
}

#[test]
fn test_holiday_get_all() {
    let my_holiday = Box::new(vec![]);
    let my_holiday = Box::into_raw(my_holiday);

    let result = unsafe { super::holiday_get_all(2020, my_holiday) };

    let holiday = unsafe { &*my_holiday };

    assert!(!result);
    assert!(holiday.len() == 11);
}

#[test]
fn test_is_holiday() {
    let t = Date {
        year: 2020,
        month: 1,
        day: 1,
    };
    let f = Date {
        year: 2020,
        month: 1,
        day: 2,
    };

    let base = HasHolidayResult {
        has_holiday: false,
        holiday: std::ptr::null(),
    };

    let value_t = Box::into_raw(Box::new(base.clone()));
    let value_f = Box::into_raw(Box::new(base));

    let result_t = unsafe { super::is_holiday(t, value_t) };
    let result_f = unsafe { super::is_holiday(f, value_f) };

    let value_t_final = unsafe { Box::from_raw(value_t) };
    let value_f_final = unsafe { Box::from_raw(value_f) };

    assert!(!result_t);
    assert!(!result_f);

    assert!(value_t_final.has_holiday);
    assert!(!value_f_final.has_holiday);

    let value_t = unsafe { &*value_t_final.holiday };
    assert!(value_t.date.day == 1);
    assert!(value_t.date.month == 1);
    assert!(value_t.date.year == 2020);

    assert!(value_f_final.holiday.is_null());
}

#[test]
fn test_get_carnival() {
    let easter = std::ptr::null_mut();
    let result_easter = unsafe { super::holidays_get_easter(2020, easter) };

    assert!(!result_easter);

    let date = unsafe { super::holiday_get_carnival(*easter) };
    assert_eq!(
        date,
        Date {
            year: 2020,
            month: 3,
            day: 31,
        }
    );
}
