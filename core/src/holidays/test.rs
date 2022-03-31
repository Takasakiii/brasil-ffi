use chrono::{Datelike, TimeZone, Utc};

use super::Easter;

#[test]
fn test_easter() {
    let tests_easter = [(2000, 4, 23), (2001, 4, 15), (2010, 4, 4), (2005, 3, 27)];
    for (year, month, day) in tests_easter.iter() {
        let easter = super::get_easter(*year).unwrap();
        assert_eq!(easter.0.month(), *month);
        assert_eq!(easter.0.day(), *day);
        assert_eq!(easter.0.year(), *year as i32);
    }
}

fn setup_easter_derivate(years: &[i32]) -> Vec<Easter> {
    years
        .iter()
        .map(|year| super::get_easter(*year).unwrap())
        .collect::<Vec<_>>()
}

#[test]
fn test_carnival() {
    let test_carnival = [(2000, 3, 7), (2001, 2, 27), (2010, 2, 16), (2005, 2, 8)];
    let years = test_carnival
        .iter()
        .map(|(year, _, _)| *year)
        .collect::<Vec<_>>();
    let easters = setup_easter_derivate(&years);
    for (index, (year, month, day)) in test_carnival.iter().enumerate() {
        let carnival = super::get_carnival(&easters[index]);
        assert_eq!(carnival.month(), *month);
        assert_eq!(carnival.day(), *day);
        assert_eq!(carnival.year(), *year as i32);
    }
}

#[test]
fn test_corpus_christi() {
    let test_carnival = [(2000, 6, 22), (2001, 6, 14), (2010, 6, 3), (2005, 5, 26)];
    let years = test_carnival
        .iter()
        .map(|(year, _, _)| *year)
        .collect::<Vec<_>>();
    let easters = setup_easter_derivate(&years);
    for (index, (year, month, day)) in test_carnival.iter().enumerate() {
        let carnival = super::get_corpus_christi(&easters[index]);
        assert_eq!(carnival.month(), *month);
        assert_eq!(carnival.day(), *day);
        assert_eq!(carnival.year(), *year as i32);
    }
}

#[test]
pub fn test_is_holyday() {
    let test_is_holiday = [
        (2001, 6, 14, true),
        (2010, 10, 10, false),
        (2020, 12, 20, false),
        (2005, 5, 26, true),
    ];

    test_is_holiday
        .into_iter()
        .for_each(|(year, month, day, is_holiday)| {
            let date = Utc.ymd(year, month, day);
            let result = super::is_holiday(date);

            assert_eq!(result.is_ok(), is_holiday);
        })
}
