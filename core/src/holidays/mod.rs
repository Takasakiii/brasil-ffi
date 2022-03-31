pub mod error;
#[cfg(test)]
mod test;

use chrono::{Date, Datelike, Duration, TimeZone, Utc};

use self::error::Error;

const EASTER_MOON_DAYS: &[(u8, u8)] = &[
    (4_u8, 14_u8),
    (4_u8, 3_u8),
    (4_u8, 23_u8),
    (4_u8, 11_u8),
    (3_u8, 31_u8),
    (4_u8, 18_u8),
    (4_u8, 8_u8),
    (3_u8, 28_u8),
    (4_u8, 16_u8),
    (4_u8, 5_u8),
    (3_u8, 25_u8),
    (4_u8, 13_u8),
    (4_u8, 2_u8),
    (3_u8, 22_u8),
    (4_u8, 10_u8),
    (3_u8, 30_u8),
    (4_u8, 17_u8),
    (4_u8, 7_u8),
    (3_u8, 27_u8),
];

pub struct Easter(Date<Utc>);

impl Easter {
    pub fn get_date(&self) -> Date<Utc> {
        self.0
    }
}

pub fn get_easter(year: i32) -> Result<Easter, Error> {
    if !(1900..=2199).contains(&year) {
        return Err(Error::InvalidYear);
    }

    let (month, day) = &EASTER_MOON_DAYS[(year % 19) as usize];
    let moving_date = Utc.ymd(year as i32, *month as u32, *day as u32);
    let day_of_week = moving_date.weekday().num_days_from_sunday();
    let next_easter = (moving_date + Duration::days(7)) - Duration::days(day_of_week as i64);

    Ok(Easter(next_easter))
}

pub fn get_carnival(easter: &Easter) -> Date<Utc> {
    easter.0 - Duration::days(47)
}

pub fn get_corpus_christi(easter: &Easter) -> Date<Utc> {
    easter.0 + Duration::days(60)
}

pub fn get_new_year(year: i32) -> Date<Utc> {
    Utc.ymd(year, 1, 1)
}

pub fn get_christmas(year: i32) -> Date<Utc> {
    Utc.ymd(year, 12, 25)
}

pub fn get_tiradentes(year: i32) -> Date<Utc> {
    Utc.ymd(year, 4, 21)
}

pub fn get_workers(year: i32) -> Date<Utc> {
    Utc.ymd(year, 5, 1)
}

pub fn get_independence(year: i32) -> Date<Utc> {
    Utc.ymd(year, 7, 9)
}

pub fn get_nossa_senhora_aparecida(year: i32) -> Date<Utc> {
    Utc.ymd(year, 10, 12)
}

pub fn get_finados(year: i32) -> Date<Utc> {
    Utc.ymd(year, 11, 2)
}

pub fn get_procration_of_the_republic(year: i32) -> Date<Utc> {
    Utc.ymd(year, 11, 15)
}

pub fn get_all_holidays(date: Date<Utc>) -> Result<[(Date<Utc>, String); 11], Error> {
    let easter = get_easter(date.year())?;

    let dates = [
        (easter.get_date(), "Páscoa"),
        (get_carnival(&easter), "Carnaval"),
        (get_corpus_christi(&easter), "Corpus Christi"),
        (get_new_year(date.year()), "Confraternização mundial"),
        (get_christmas(date.year()), "Natal"),
        (get_tiradentes(date.year()), "Tiradentes"),
        (get_workers(date.year()), "Dia do trabalho"),
        (get_independence(date.year()), "Independência do Brasil"),
        (
            get_nossa_senhora_aparecida(date.year()),
            "Nossa Senhora Aparecida",
        ),
        (get_finados(date.year()), "Finados"),
        (
            get_procration_of_the_republic(date.year()),
            "Procriação da República",
        ),
    ]
    .map(|(date, name)| (date, name.to_string()));

    Ok(dates)
}

pub fn is_holiday(date: Date<Utc>) -> Result<String, Error> {
    for (holiday_date, holiday_name) in get_all_holidays(date)?.into_iter() {
        if holiday_date == date {
            return Ok(holiday_name);
        }
    }

    Err(Error::NotHoliday)
}

pub fn get_date(timestamp_seconds: i64) -> Date<Utc> {
    Utc.timestamp(timestamp_seconds, 0).date()
}
