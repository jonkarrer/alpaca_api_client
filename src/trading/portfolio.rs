use super::AccountType;
use crate::{request, TimeFrame};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PortfolioHistory {
    pub timestamp: Vec<i64>,
    pub equity: Vec<f64>,
    pub profit_loss: Vec<f64>,
    pub profit_loss_pct: Vec<f64>,
    pub base_value: f64,
    pub base_value_asof: Option<String>,
    pub timeframe: String,
    pub cashflow: Option<String>,
}

pub struct PortfolioHistoryQuery<'a> {
    pub url: &'a str,
    pub period: Option<&'a str>,
    pub timeframe: Option<TimeFrame>,
    pub intraday_reporting: Option<&'a str>,
    pub start: Option<&'a str>,
    pub end: Option<&'a str>,
    pub pnl_reset: Option<&'a str>,
    pub date_end: Option<&'a str>,
    pub extended_hours: Option<&'a str>,
    pub cashflow_types: Option<Vec<&'a str>>,
}

impl<'a> PortfolioHistoryQuery<'a> {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            url: match account_type {
                AccountType::Live => "https://api.alpaca.markets/v2/account/portfolio/history",
                AccountType::Paper => {
                    "https://paper-api.alpaca.markets/v2/account/portfolio/history"
                }
            },
            period: None,
            timeframe: None,
            intraday_reporting: None,
            start: None,
            end: None,
            pnl_reset: None,
            date_end: None,
            extended_hours: None,
            cashflow_types: None,
        }
    }

    pub fn period(mut self, period: &'a str) -> Self {
        self.period = Some(period);
        self
    }

    pub fn timeframe(mut self, timeframe: TimeFrame) -> Self {
        self.timeframe = Some(timeframe);
        self
    }

    pub fn intraday_reporting(mut self, intraday_reporting: &'a str) -> Self {
        self.intraday_reporting = Some(intraday_reporting);
        self
    }

    pub fn start(mut self, start: &'a str) -> Self {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: &'a str) -> Self {
        self.end = Some(end);
        self
    }

    pub fn pnl_reset(mut self, pnl_reset: &'a str) -> Self {
        self.pnl_reset = Some(pnl_reset);
        self
    }

    pub fn date_end(mut self, date_end: &'a str) -> Self {
        self.date_end = Some(date_end);
        self
    }

    pub fn extended_hours(mut self, extended_hours: &'a str) -> Self {
        self.extended_hours = Some(extended_hours);
        self
    }

    pub fn cashflow_types(mut self, cashflow_types: Vec<&'a str>) -> Self {
        self.cashflow_types = Some(cashflow_types);
        self
    }

    fn build(self) -> String {
        let mut query = String::new();
        if let Some(period) = self.period {
            query.push_str(&format!("&period={}", period));
        }
        if let Some(timeframe) = self.timeframe {
            query.push_str(&format!("&timeframe={}", timeframe.to_string()));
        }
        if let Some(intraday_reporting) = self.intraday_reporting {
            query.push_str(&format!("&intraday_reporting={}", intraday_reporting));
        }
        if let Some(start) = self.start {
            query.push_str(&format!("&start={}", start));
        }
        if let Some(end) = self.end {
            query.push_str(&format!("&end={}", end));
        }
        if let Some(pnl_reset) = self.pnl_reset {
            query.push_str(&format!("&pnl_reset={}", pnl_reset));
        }
        if let Some(date_end) = self.date_end {
            query.push_str(&format!("&date_end={}", date_end));
        }
        if let Some(extended_hours) = self.extended_hours {
            query.push_str(&format!("&extended_hours={}", extended_hours));
        }
        if let Some(cashflow_types) = self.cashflow_types {
            query.push_str(&format!("&cashflow_types={}", cashflow_types.join(",")));
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<PortfolioHistory, ureq::Error> {
        let url = self.build();
        let response = request("GET", &url).call()?;
        Ok(response.into_body().read_json()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_portfolio_history() {
        let query = PortfolioHistoryQuery::new(AccountType::Paper)
            .send()
            .unwrap();

        dbg!(&query);
        assert!(query.timestamp.len() > 0);
    }
}
