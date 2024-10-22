use std::{io::Error, str::FromStr};

/// Timeframe options utility for time frames accepted by alpaca api
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TimeFrame {
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    FourHours,
    OneDay,
    OneWeek,
    OneMonth,
}

impl ToString for TimeFrame {
    fn to_string(&self) -> String {
        match self {
            TimeFrame::OneMinute => "1Min".to_string(),
            TimeFrame::FiveMinutes => "5Min".to_string(),
            TimeFrame::FifteenMinutes => "15Min".to_string(),
            TimeFrame::ThirtyMinutes => "30Min".to_string(),
            TimeFrame::OneHour => "1H".to_string(),
            TimeFrame::FourHours => "4H".to_string(),
            TimeFrame::OneDay => "1D".to_string(),
            TimeFrame::OneWeek => "1W".to_string(),
            TimeFrame::OneMonth => "1M".to_string(),
        }
    }
}

impl FromStr for TimeFrame {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1Min" => Ok(TimeFrame::OneMinute),
            "5Min" => Ok(TimeFrame::FiveMinutes),
            "15Min" => Ok(TimeFrame::FifteenMinutes),
            "30Min" => Ok(TimeFrame::ThirtyMinutes),
            "1Hour" => Ok(TimeFrame::OneHour),
            "4Hour" => Ok(TimeFrame::FourHours),
            "1Day" => Ok(TimeFrame::OneDay),
            "1Week" => Ok(TimeFrame::OneWeek),
            "1Month" => Ok(TimeFrame::OneMonth),
            "1T" => Ok(TimeFrame::OneMinute),
            "5T" => Ok(TimeFrame::FiveMinutes),
            "15T" => Ok(TimeFrame::FifteenMinutes),
            "30T" => Ok(TimeFrame::ThirtyMinutes),
            "1H" => Ok(TimeFrame::OneHour),
            "4H" => Ok(TimeFrame::FourHours),
            "1D" => Ok(TimeFrame::OneDay),
            "1W" => Ok(TimeFrame::OneWeek),
            "1M" => Ok(TimeFrame::OneMonth),
            _ => Err(Error::other("Timeframes Do Not Match")),
        }
    }
}
