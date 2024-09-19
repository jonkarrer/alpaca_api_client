use super::AccountType;
use crate::request;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CalendarDay {
    pub date: String,
    pub open: String,
    pub close: String,
    pub settlement_date: String,
}

pub type Calendar = Vec<CalendarDay>;

pub struct CalendarQuery<'a> {
    url: &'a str,
    start: Option<&'a str>,
    end: Option<&'a str>,
    date_type: Option<&'a str>,
}

impl<'a> CalendarQuery<'a> {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            url: match account_type {
                AccountType::Live => "https://api.alpaca.markets/v2/calendar",
                AccountType::Paper => "https://paper-api.alpaca.markets/v2/calendar",
            },
            start: None,
            end: None,
            date_type: None,
        }
    }

    pub fn start(mut self, start: &'a str) -> Self {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: &'a str) -> Self {
        self.end = Some(end);
        self
    }

    pub fn date_type(mut self, date_type: &'a str) -> Self {
        self.date_type = Some(date_type);
        self
    }

    fn build(self) -> String {
        let mut query = String::new();
        if let Some(start) = self.start {
            query.push_str(&format!("&start={}", start));
        }
        if let Some(end) = self.end {
            query.push_str(&format!("&end={}", end));
        }
        if let Some(date_type) = self.date_type {
            query.push_str(&format!("&date_type={}", date_type));
        }
        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<Calendar, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let response: Calendar = response.into_json()?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calendar_query() {
        let query = CalendarQuery::new(AccountType::Paper).send().unwrap();

        dbg!(&query);
        assert!(query.len() > 0);
    }
}
