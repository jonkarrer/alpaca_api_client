use std::string;

use serde::Deserialize;

use crate::request;

#[derive(Deserialize, Debug)]
pub struct NewsArticle {
    pub author: String,
    pub content: String,
    pub created_at: String,
    pub headline: String,
    pub id: i32,
    pub images: Vec<NewsImage>,
    pub source: String,
    pub summary: String,
    pub symbols: Vec<String>,
    pub updated_at: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct NewsImage {
    pub url: String,
    pub size: String,
}

pub type News = Vec<NewsArticle>;

#[derive(Deserialize, Debug)]
pub struct NewsResponse {
    pub news: News,
    pub next_page_token: Option<String>,
}

pub struct NewsQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    start: Option<&'a str>,
    end: Option<&'a str>,
    limit: Option<i32>,
    include_content: Option<bool>,
    exclude_contentless: Option<bool>,
    sort_desc: bool,
    sort_asc: bool,
}

impl<'a> NewsQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta1/news",
            symbols,
            start: None,
            end: None,
            limit: None,
            include_content: None,
            exclude_contentless: None,
            sort_desc: false,
            sort_asc: false,
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

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn include_content(mut self, include_content: bool) -> Self {
        self.include_content = Some(include_content);
        self
    }

    pub fn exclude_contentless(mut self, exclude_contentless: bool) -> Self {
        self.exclude_contentless = Some(exclude_contentless);
        self
    }

    pub fn sort_desc(mut self, sort_desc: bool) -> Self {
        self.sort_desc = true;
        self.sort_asc = false;
        self
    }

    pub fn sort_asc(mut self, sort_asc: bool) -> Self {
        self.sort_asc = true;
        self.sort_desc = false;
        self
    }

    fn build(&self) -> String {
        let mut query = format!("symbols={}", self.symbols.join(","));
        if let Some(start) = self.start {
            query.push_str(&format!("&start={}", start));
        }
        if let Some(end) = self.end {
            query.push_str(&format!("&end={}", end));
        }
        if let Some(limit) = self.limit {
            query.push_str(&format!("&limit={}", limit));
        }
        if let Some(include_content) = self.include_content {
            query.push_str(&format!("&include_content={}", include_content));
        }
        if let Some(exclude_contentless) = self.exclude_contentless {
            query.push_str(&format!("&exclude_contentless={}", exclude_contentless));
        }
        if self.sort_asc {
            query.push_str("&sort=asc");
        } else if self.sort_desc {
            query.push_str("&sort=desc");
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(&self) -> Result<News, ureq::Error> {
        let route = self.build();
        let mut news = Vec::new();
        let mut page_token = None;

        let mut i = 0;
        let data_limit = if let Some(limit) = self.limit {
            limit
        } else {
            50
        };
        loop {
            if i >= data_limit {
                break;
            }

            // If a token exists, append to address
            let temp_address = match page_token {
                Some(token) => format!("{}&page_token={}", &route, &token),
                _ => route.clone(),
            };
            let response = request("GET", &temp_address).call()?;
            let response: NewsResponse = response.into_json()?;
            i += response.news.len() as i32;
            news.extend(response.news);

            // If a token is in response, assign to page_token for next loop
            match response.next_page_token {
                Some(next_page_token) => page_token = Some(next_page_token.clone()),
                _ => break,
            }
        }

        Ok(news)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_query() {
        let symbols = vec!["AAPL"];
        let query = NewsQuery::new(symbols)
            .include_content(true)
            .exclude_contentless(true)
            .limit(10)
            .sort_desc(true)
            .send()
            .unwrap();

        dbg!(&query);
        assert!(query.len() == 10);
    }
}
