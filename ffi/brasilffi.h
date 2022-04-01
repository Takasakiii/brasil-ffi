#pragma once

typedef void Easter;

typedef struct
{
    int year;
    int unsigned month;
    int unsigned day;
} Date;

typedef struct
{
    char *name;
    Date date;
} Holiday;

typedef struct
{
    int has_holiday;
    Holiday *holiday;
} HasHolidayResult;

int holidays_get_easter(int year, Easter **easter);
Date holiday_easter_into_date(Easter *easter);
Date holiday_get_carnival(Easter *easter);
Date holiday_get_corpus_christi(Easter *easter);

Date holiday_get_new_year(int year);
Date holiday_get_christmas(int year);
Date holiday_get_tiradentes(int year);
Date holiday_get_workers(int year);
Date holiday_get_independence(int year);
Date holiday_get_nossa_senhora_aparecida(int year);
Date holiday_get_finados(int year);
Date holiday_get_procration_of_the_republic(int year);

int holiday_get_all(int year, Holiday **holidays); // returns array of holidays
int is_holiday(Date date, HasHolidayResult *result);