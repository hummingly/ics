use std::ops::RangeInclusive;
use value_data_type::Date;
use value_data_type::DateTime;

const SECOND_NUM: RangeInclusive<u8> = 0..=60;
const MINUTE_NUM: RangeInclusive<u8> = 0..=59;
const HOUR_NUM: RangeInclusive<u8> = 0..=23;
const YEAR_DAY_NUM: RangeInclusive<u16> = 1..=366;
const WEEK_NUM: RangeInclusive<u8> = 1..=53;
const MONTH_NUM: RangeInclusive<u8> = 1..=12;
const MONTH_DAY_NUM: RangeInclusive<u8> = 1..=31;

pub enum DurationValue {
    Date(usize, u32),
    Time(u32),
    Week(usize)
}

pub enum Sign {
    Plus,
    Minus
}

pub enum RecurRule {
    FREQ(Freq),
    UNTIL(Until),
    COUNT(usize),
    INTERVAL(usize),
    BYSECOND(Vec<u8>),
    BYMINUTE(Vec<u8>),
    BYHOUR(Vec<u8>),
    BYDAY(Vec<WeekDayNum>),
    BYMONTHDAY(Vec<MonthDayNum>),
    BYYEARDAY(Vec<YearDayNum>),
    BYWEEKNO(Vec<WeekNum>),
    BYMONTH(Vec<MonthNum>),
    BYSETPOS(Vec<YearDayNum>),
    WKST(WeekDay)
}

pub enum Freq {
    SECONDLY,
    MINUTELY,
    HOURLY,
    DAILY,
    WEEKLY,
    MONTHLY,
    YEARLY
}

pub enum Until {
    Date(Date),
    DateTime(DateTime)
}

pub struct WeekDayNum {
    sign: Option<Sign>,
    ord_week: Option<u8>,
    weekday: WeekDay
}

pub enum WeekDay {
    SU,
    MO,
    TU,
    WE,
    TH,
    FR,
    SA
}

pub struct WeekNum {
    sign: Option<Sign>,
    ord_week: u8
}

pub struct MonthNum(u8);

pub struct MonthDayNum {
    sign: Option<Sign>,
    ord_monthday: u8
}

pub struct YearDayNum {
    sign: Option<Sign>,
    ord_yearday: u16
}
