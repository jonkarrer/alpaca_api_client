use super::AccountType;
use crate::request;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TradeActivity {
    pub activity_type: Option<String>,
    pub id: String,
    pub cum_qty: Option<String>,
    pub leaves_qty: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
    pub side: Option<String>,
    pub symbol: Option<String>,
    pub transaction_time: Option<String>,
    pub order_id: Option<String>,
    pub r#type: Option<String>,
    pub order_status: Option<String>,
    pub date: Option<String>,
    pub net_amount: Option<String>,
    pub per_share_amount: Option<String>,
    pub group_id: Option<String>,
    pub status: Option<String>,
}

pub type TradeActivities = Vec<TradeActivity>;

pub struct ActivitiesQuery<'a> {
    url: &'a str,
    activity_types: Option<Vec<&'a str>>,
    category: Option<&'a str>,
    date: Option<&'a str>,
    until: Option<&'a str>,
    after: Option<&'a str>,
    direction: Option<&'a str>,
    page_size: Option<usize>,
    limit: Option<usize>,
}

impl<'a> ActivitiesQuery<'a> {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            url: match account_type {
                AccountType::Live => "https://api.alpaca.markets/v2/account/activities",
                AccountType::Paper => "https://paper-api.alpaca.markets/v2/account/activities",
            },
            activity_types: None,
            category: None,
            date: None,
            until: None,
            after: None,
            direction: None,
            page_size: None,
            limit: None,
        }
    }

    pub fn activity_types(mut self, activity_types: Vec<&'a str>) -> Self {
        self.activity_types = Some(activity_types);
        self.category = None;
        self
    }

    pub fn category(mut self, category: &'a str) -> Self {
        self.category = Some(category);
        self.activity_types = None;
        self
    }

    pub fn date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }

    pub fn until(mut self, until: &'a str) -> Self {
        self.until = Some(until);
        self
    }

    pub fn after(mut self, after: &'a str) -> Self {
        self.after = Some(after);
        self
    }

    pub fn direction(mut self, direction: &'a str) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn page_size(mut self, page_size: usize) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    fn build(&self) -> String {
        let mut query = String::new();
        if let Some(activity_types) = &self.activity_types {
            query.push_str(&format!("&activity_types={}", activity_types.join(",")));
        }
        if let Some(category) = self.category {
            query.push_str(&format!("&category={}", category));
        }
        if let Some(date) = self.date {
            query.push_str(&format!("&date={}", date));
        }
        if let Some(until) = self.until {
            query.push_str(&format!("&until={}", until));
        }
        if let Some(after) = self.after {
            query.push_str(&format!("&after={}", after));
        }
        if let Some(direction) = self.direction {
            query.push_str(&format!("&direction={}", direction));
        }
        if let Some(page_size) = self.page_size {
            query.push_str(&format!("&page_size={}", page_size));
        }
        format!("{}?{}", self.url, query)
    }

    pub fn send(&self) -> Result<TradeActivities, ureq::Error> {
        let route = self.build();
        let mut trade_activities: TradeActivities = Vec::new();
        let mut page_token = None;

        let mut i = 0;
        let data_limit = if let Some(limit) = self.limit {
            limit
        } else {
            1000
        };

        let expected_page_size = if let Some(size) = self.page_size {
            size
        } else {
            100
        };

        loop {
            if i >= data_limit {
                break;
            }

            // If a token exists, append to address
            let temp_address = match page_token {
                Some(ref token) => format!("{}&page_token={}", &route, &token),
                _ => route.clone(),
            };

            let response = request("GET", &temp_address).call()?;
            let response: TradeActivities = response.into_body().read_json()?;
            let returned_page_size = response.len();

            for (index, item) in response.into_iter().enumerate() {
                i += 1;
                if index == returned_page_size && returned_page_size == expected_page_size {
                    page_token = Some(item.id.clone());
                }
                trade_activities.push(item);
            }

            if page_token.is_none() {
                break;
            }
        }

        Ok(trade_activities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_activities_query() {
        let query = ActivitiesQuery::new(AccountType::Paper).send().unwrap();

        dbg!(&query);
        assert!(query.len() > 0);
    }
}
