# Alpaca API Client in Rust

This is an unofficial SDK for the Alpaca API written in Rust. <a href="https://alpaca.markets/">Alpaca</a> is a trading platform for developers and app makers,
and they provide various endpoints to access over http. The goal of this package is to provide
the basic functionality for using the Alpaca API. This is a work in progress, and will need contributors to take this all the way.

**Still a Work In Progress**
Recommended for Paper Accounts only.

![Build Status](https://img.shields.io/badge/build-passing-green.svg)  ![Version 0.6.0](https://img.shields.io/badge/version-0.6.0-blue.svg)

## Installation

```bash
cargo add alpaca_api_client
```

We use dotenvy to pull in the API keys. Add your API keys to an <b>.env</b> file in the root of your directory with these names.

```bash
/.env
APCA_API_KEY_ID=<pub_key>
APCA_API_SECRET_KEY=<secret_key>
```

Or provide them in the environment variables.

```bash
APCA_API_KEY_ID=<pub_key>
APCA_API_SECRET_KEY=<secret_key>
```

## Features

Currently, this package only provides the ability to interact with the Market Data and Trading APIs. The Broker API is not yet supported.

### Market Data

Link to the documentation -> [Alpaca API Docs](https://docs.alpaca.markets/docs/about-market-data-api)

- Stocks
- Crypto
- News
- Options
- Screener

### Trading

Link to the documentation -> [Alpaca API Docs](https://docs.alpaca.markets/docs/trading-api)

- Account
- Activities
- Assets
- Calendar
- Clock
- Order
- Positions
- Portfolio

## Usage

[RS Docs](https://docs.rs/alpaca_api_client/0.6.0/alpaca_api_client/) | [Examples](https://github.com/jonkarrer/alpaca_api_client/blob/main/src/account.rs)

The Alpaca api is mostly designed to be manipulated through query parameters. There are some cases where they want a body in the request with our desired queries. In either case, this library operates by allowing users to manage their queries with a builder pattern. Essentially, the query params map to setters on the query builder. Here are a few examples.

### Historical Bars

```rust
use alpaca_api_client::{market_data::stocks::bars::HistoricalBarsQuery, TimeFrame};

let query = HistoricalBarsQuery::new(vec!["AAPL"], TimeFrame::OneDay)
    .start("2022-02-01") // date to start
    .end("2022-02-10") // date to end
    .feed("iex") // feed to use (iex is free, sip is premium). See Alpaca docs for more info
    .send() // this finalizes the query and send it off
    .unwrap();

dbg!(&query);
```

<details>

<summary>See output</summary>

<br />

```rust
{
    "TSLA": [
        StockBar {
            t: "2022-02-01T05:00:00Z",
            o: 934.4,
            h: 943.35,
            l: 905.02,
            c: 930.48,
            v: 651095.0,
            n: 17534,
            vw: 926.5979,
        },
        StockBar {
            t: "2022-02-02T05:00:00Z",
            o: 926.58,
            h: 931.35,
            l: 889.48,
            c: 905.64,
            v: 542975.0,
            n: 14412,
            vw: 908.74176,
        },
    ],
    "AAPL": [
        StockBar {
            t: "2022-02-01T05:00:00Z",
            o: 174.0,
            h: 174.78,
            l: 172.36,
            c: 174.54,
            v: 1094581.0,
            n: 10621,
            vw: 173.47993,
        },
        StockBar {
            t: "2022-02-02T05:00:00Z",
            o: 174.82,
            h: 175.88,
            l: 173.33,
            c: 175.58,
            v: 1304266.0,
            n: 11695,
            vw: 174.82686,
        },
    ],
}
```

</details>

### News

```rust
let query = NewsQuery::new(vec!["AAPL"])
    .include_content(true) // include content in the response
    .exclude_contentless(true) // exclude content less articles
    .limit(2) // limit the number of articles to 2
    .sort_desc() // sort the articles in descending order
    .send() // this finalizes the query and send it off
    .unwrap();

dbg!(&query);
```

<details>

<summary>See output</summary>

<br />

```rust
[
    NewsArticle {
        author: "The Arora Report",
        content: "<figure class=\"wp-block-image size-large\"><img decoding=\"async\" src=\"https://thearorareport.com/wp-content/uploads/2024/09/2024-09-20_08h35_26-1.png\" alt=\"\"/></figure>\n\n\n\n<p><em>To gain an edge, this is what you need to know today.</em></p>\n\n\n\n<h3 class=\"wp-block-heading\"><strong>Nuclear Power Revival</strong></h3>\n\n\n\n<p>Please <a href=\"https://thearorareport.com/chart-analysis-stock-market-quad-witching-nuclear-power-revival-ceg-mc-09202024\">click here</a> for an enlarged chart of <strong>Constellation Energy Corp</strong> (NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/CEG#NASDAQ\">CEG</a>).</p>\n\n\n\n<p>Note the following:</p>\n\n\n\n<ul>\n<li>This article is about the big picture, not an individual stock.&nbsp; The chart of CEG is being used to illustrate the point.</li>\n\n\n\n<li>To feed the power hungry AI data centers, <strong>Microsoft Corp </strong>(NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/MSFT#NASDAQ\">MSFT</a>) is signing a power purchase agreement with CEG.</li>\n\n\n\n<li>The chart shows the gap up on CEG news.</li>\n\n\n\n<li>The chart shows that CEG is in the resistance zone.</li>\n\n\n\n<li>RSI on the chart shows that CEG is very overbought.&nbsp; An overbought stock tends to pullback.</li>\n\n\n\n<li>Based on the quantitative screen of The Arora Report's ZYX Change Method, CEG is way overvalued at this time.&nbsp; However, CEG will benefit from AI, and as such, it is on our radar to give a signal when it is appropriate.</li>\n\n\n\n<li>To provide the power, CEG will restart Three Mile Island Unit 1.&nbsp; This reactor was shut down about five years ago due to economic reasons.&nbsp; Three Mile Island Unit 2 is famous for a partial melt down, which caused the most serious nuclear accident on U.S. soil.&nbsp; In addition to CEG, there are opportunities in other energy producers that will help meet the massive power demand for AI.\n<ul>\n<li><strong>Quanta Services Inc</strong> (NYSE:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/PWR#NYSE\">PWR</a>) is a contractor to electric utilities and is benefiting from the AI power demand.&nbsp; As full disclosure, PWR is in The Arora Report&#8217;s ZYX Buy Core Model Portfolio. PWR is long from $37.&nbsp; </li>\n\n\n\n<li><strong>First Solar Inc</strong> (NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/FSLR#NASDAQ\">FSLR</a>), which provides solar power to data centers.  As full disclosure, FSLR is in The Arora Report&#8217;s ZYX Buy Core Model Portfolio.</li>\n\n\n\n<li>A new name that is on The Arora Report&#8217;s radar is <strong>Vistra Corp</strong> (NYSE:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/VST#NYSE\">VST</a>).  As full disclosure, a signal may be given when appropriate.</li>\n</ul>\n</li>\n\n\n\n<li>We shared with readers yesterday that quad witching was to the upside.\n<ul>\n<li>A part of the strength in the stock market yesterday was due to quad witching and not due to the Fed's 50 bps interest rate cut.</li>\n\n\n\n<li>About $5 trillion notional value of stock derivatives are expiring today.&nbsp; This is a huge amount but not a record.</li>\n\n\n\n<li>This morning, expiring futures are exerting downward pressure on the market after yesterday's runup.&nbsp; Futures are expiring this morning.</li>\n\n\n\n<li>Options will expire in the afternoon.</li>\n</ul>\n</li>\n\n\n\n<li>Historically, gains due to quad witching tend to reverse in the following week and sometimes even on the expiration day.</li>\n</ul>\n\n\n\n<h3 class=\"wp-block-heading\"><strong>Japan</strong></h3>\n\n\n\n<p>In a relief to U.S. stock investors, the Bank of Japan (BOJ) left interest rates unchanged.</p>\n\n\n\n<h3 class=\"wp-block-heading\"><strong>Magnificent Seven Money Flows</strong></h3>\n\n\n\n<p>In the early trade, money flows are positive in <strong>Alphabet Inc Class C</strong> (NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/GOOG#NASDAQ\">GOOG</a>).</p>\n\n\n\n<p>In the early trade, money flows are neutral in <strong>Apple Inc </strong>(NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/AAPL#NASDAQ\">AAPL</a>), <strong>Amazon.com, Inc.</strong> (NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/AMZN#NASDAQ\">AMZN</a>), <strong>Meta Platforms Inc</strong> (NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/META#NASDAQ\">META</a>), and MSFT.</p>\n\n\n\n<p>In the early trade, money flows are negative in <strong>NVIDIA Corp</strong> (NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/NVDA#NASDAQ\">NVDA</a>), and <strong>Tesla Inc</strong> (NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/TSLA#NASDAQ\">TSLA</a>).</p>\n\n\n\n<p>In the early trade, money flows are negative in <strong>SPDR S&amp;P 500 ETF Trust</strong> (NYSE:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/SPY#NYSE\">SPY</a>) and <strong>Invesco QQQ Trust Series 1</strong> (NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/QQQ#NASDAQ\">QQQ</a>).</p>\n\n\n\n<h4 class=\"wp-block-heading\"><strong>Momo Crowd And Smart Money In Stocks</strong></h4>\n\n\n\n<p>Investors can gain an edge by knowing money flows in SPY and QQQ.\u{a0} Investors can get a bigger edge by knowing when smart money is buying stocks, gold, and oil.\u{a0} The most popular ETF for gold is <strong>SPDR Gold Trust</strong> (NYSE:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/GLD#NYSE\">GLD</a>).\u{a0} The most popular ETF for silver is <strong>iShares Silver Trust </strong>(NYSE:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/SLV#NYSE\">SLV</a>).\u{a0} The most popular ETF for oil is <strong>United States Oil ETF</strong> (NYSE:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/USO#NYSE\">USO</a>).</p>\n\n\n\n<h3 class=\"wp-block-heading\"><strong>Bitcoin</strong></h3>\n\n\n\n<p>Trump's advocacy continues to bring buying into <strong>Bitcoin</strong> (CRYPTO: <a class=\"ticker\" href=\"https://www.benzinga.com/quote/btc/usd\">BTC</a>). Here is the key question: Will bitcoin whales take advantage of the strength to sell as is their pattern?</p>\n\n\n\n<h3 class=\"wp-block-heading\"><strong>Protection Band And What To Do Now</strong></h3>\n\n\n\n<p>It is important for investors to look ahead and not in the rearview mirror.</p>\n\n\n\n<p>Consider continuing to hold good, very long term, existing positions. Based on individual risk preference, consider a protection band consisting of cash or Treasury bills or short-term tactical trades as well as short to medium term hedges and short term hedges. This is a good way to protect yourself and participate in the upside at the same time.</p>\n\n\n\n<p>You can determine your protection bands by adding cash to hedges.&nbsp; The high band of the protection is appropriate for those who are older or conservative. The low band of the protection is appropriate for those who are younger or aggressive.&nbsp; If you do not hedge, the total cash level should be more than stated above but significantly less than cash plus hedges.</p>\n\n\n\n<p>A protection band of 0% would be very bullish and would indicate full investment with 0% in cash.&nbsp; A protection band of 100% would be very bearish and would indicate a need for aggressive protection with cash and hedges or aggressive short selling.</p>\n\n\n\n<p><strong>It is worth reminding that you cannot take advantage of new upcoming opportunities if you are not holding enough cash.&nbsp; </strong>When adjusting hedge levels, consider adjusting partial stop quantities for stock positions (non ETF); consider using wider stops on remaining quantities and also allowing more room for high beta stocks.&nbsp; High beta stocks are the ones that move more than the market.</p>\n\n\n\n<h3 class=\"wp-block-heading\"><strong>Traditional 60/40 Portfolio</strong></h3>\n\n\n\n<p>Probability based risk reward adjusted for inflation does not favor long duration strategic bond allocation at this time.</p>\n\n\n\n<p>Those who want to stick to traditional 60% allocation to stocks and 40% to bonds may consider focusing on only high quality bonds and bonds of five year duration or less.&nbsp; Those willing to bring sophistication to their investing may consider using bond ETFs as tactical positions and not strategic positions at this time.</p>\n\n\n\n<p><strong>The Arora Report is known for its accurate calls. The Arora Report correctly called the big artificial intelligence rally before anyone else, the new bull market of 2023, the bear market of 2022, new stock market highs right after the virus low in 2020, the virus drop in 2020, the DJIA rally to 30,000 when it was trading at 16,000, the start of a mega bull market in 2009, and the financial crash of 2008. Please click here to sign up for a free forever </strong><a href=\"https://thearorareport.com/generate-wealth-newsletter-benzinga\"><strong>Generate Wealth Newsletter</strong></a><strong>.</strong></p>    ",
        created_at: "2024-09-20T14:28:58Z",
        headline: "$5 Trillion Worth Of Quad Witching, AI Leading To Revivial Of Nuclear Power",
        id: 40959279,
        images: [
            NewsImage {
                url: "https://cdn.benzinga.com/files/imagecache/2048x1536xUP/images/story/2024/09/20/analyst-ratings-image-8736.jpeg",
                size: "large",
            },
            NewsImage {
                url: "https://cdn.benzinga.com/files/imagecache/1024x768xUP/images/story/2024/09/20/analyst-ratings-image-8736.jpeg",
                size: "small",
            },
            NewsImage {
                url: "https://cdn.benzinga.com/files/imagecache/250x187xUP/images/story/2024/09/20/analyst-ratings-image-8736.jpeg",
                size: "thumb",
            },
        ],
        source: "benzinga",
        summary: "\n\n\n\nTo gain an edge, this is what you need to know today.",
        symbols: [
            "AAPL",
            "AMZN",
            "BTCUSD",
            "CEG",
            "FSLR",
            "GLD",
            "GOOG",
            "META",
            "MSFT",
            "NVDA",
            "PWR",
            "QQQ",
            "SLV",
            "SPY",
            "TSLA",
            "USO",
            "VST",
        ],
        updated_at: "2024-09-20T14:29:00Z",
        url: "https://www.benzinga.com/24/09/40959279/5-trillion-worth-of-quad-witching-ai-leading-to-revivial-of-nuclear-power",
    },
    NewsArticle {
        author: "Chris Katje",
        content: "<p><strong>Apple Inc </strong>(NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/AAPL#NASDAQ\">AAPL</a>) is one of the stocks that could be volatile Friday with S&amp;P 500 and several market indexes rebalancing along with the quarterly triple witching event that sees the expiration of <a href=\"https://www.benzinga.com/analyst-ratings/analyst-color/24/09/40948624/traders-brace-for-friday-volatility-as-over-5-trillion-in-options-expire-could-trip\">stock options, index futures and index options.</a></p>\n\n\n\n<p><strong>What Happened</strong>: The S&amp;P 500, <span style=\"box-sizing: border-box; margin: 0px; padding: 0px;\"><a href=\"https://www.benzinga.com/topic/s-p-500\" target=\"_blank\" rel=\"noopener\">tracked</a>\u{a0}by the\u{a0}<strong>SPDR S&amp;P 500 ETF Trust\u{a0}</strong>(NYSE:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/SPY#NYSE\">SPY</a>), will complete a quarterly rebalance </span>on Friday. This task is traditionally done on the third Friday of March, June, September, and December.</p>\n\n\n\n<p>Apple could be one of the biggest beneficiaries of the rebalancing by the S&amp;P 500 and other market indexes.</p>\n\n\n\n<p>The technology giant is currently the second-largest holding in the SPY, with 6.7% of assets and trailing <strong>Microsoft Corporation </strong>(NASDAQ:<a class=\"ticker\" href=\"https://www.benzinga.com/stock/MSFT#NASDAQ\">MSFT</a>) at 6.8%.</p>\n\n\n\n<p>The rebalance will see Apple take the lead in the holdings, as the company's market capitalization of $3.48 trillion is ahead of Microsoft at $3.26 trillion.</p>\n\n\n\n<p>Friday's new rebalancing will coincide with upcoming S&amp;P index-capping rules that will adjust how allocations for the largest companies are determined, according to Bloomberg.</p>\n\n\n\n<p>The reshuffling of $250 billion in shares in index-tracking funds is expected to benefit the technology sector and companies like Apple the most, the report said.</p>\n\n\n\n<p>Piper Sandler estimates that the technology sector will see $40 billion in net buying, as the only sector with a net gain, according to the Bloomberg report.</p>\n\n\n\n<p>Apple will account for a large portion of the $40 billion in net buying thanks to its market capitalization, new rules, and recent sales by <strong>Warren Buffett </strong>and <strong>Berkshire Hathaway.</strong></p>\n\n\n\n<p>The sale by Berkshire Hathaway has changed the float of Apple stock, with more shares available to investors. Rules from indexes, in some cases, use float-adjusted market capitalizations, which take into account large stakes held by fund managers.</p>\n\n\n\n<p><strong>Did You Know?</strong></p>\n\n\n\n<ul>\n<li>Congress Is Making Huge Investments. <a href=\"https://www.benzinga.com/gov-trades/?utm_source=BZprocontent092024\"><strong>Get Tips On What They Bought And Sold Ahead Of The 2024 Election With Our Easy-to-Use Tool</strong></a></li>\n</ul>\n\n\n\n<p><strong>Why It's Important: </strong>Apple and Microsoft have battled back and forth over the last several years as the world's most valuable company. Both companies <a href=\"https://www.benzinga.com/news/24/06/39386946/microsoft-apple-step-aside-nvidia-is-now-the-most-valuable-public-company-in-the-world\">were briefly passed</a> by <strong>Nvidia </strong>earlier this year.</p>\n\n\n\n<p>Friday's rebalancing is expected to bring market volatility and reshape many index funds moving forward.</p>\n\n\n\n<p>For Apple, the event comes as the iPhone 16 is publicly released<span style=\"box-sizing: border-box; margin: 0px; padding: 0px;\">. Investors and analysts are expressing concerns as consumers grapple with whether the new features and the rollout of Apple Intelligence are worth\u{a0}</span>the upgrade from their existing device.</p>\n\n\n\n<p><strong>AAPL Price Action: </strong>Apple stock trades at $229.02 versus a 52-week trading range of $164.08 to $237.23. Apple stock is up 23% year-to-date in 2024.</p>\n\n\n\n<p><strong>Read Next: </strong></p>\n\n\n\n<ul>\n<li><a href=\"https://www.benzinga.com/analyst-ratings/analyst-color/24/09/40940715/apple-revenue-growth-just-too-slow-for-investors-to-get-excited-about-analyst-highl\"><strong>Apple Revenue Growth &#8216;Just Too Slow For Investors To Get Excited About&#8217;: Analyst Highlights Advertising As Future Catalyst</strong></a></li>\n</ul>\n\n\n\n<p><em>Photo: Shutterstock</em></p>",
        created_at: "2024-09-20T13:48:39Z",
        headline: "How Apple Stock Could Gain From Friday's S&P 500 Rebalancing: What Investors Should Know",
        id: 40958207,
        images: [
            NewsImage {
                url: "https://cdn.benzinga.com/files/imagecache/2048x1536xUP/images/story/2024/09/20/Major-Stake-In-Apple.jpeg",
                size: "large",
            },
            NewsImage {
                url: "https://cdn.benzinga.com/files/imagecache/1024x768xUP/images/story/2024/09/20/Major-Stake-In-Apple.jpeg",
                size: "small",
            },
            NewsImage {
                url: "https://cdn.benzinga.com/files/imagecache/250x187xUP/images/story/2024/09/20/Major-Stake-In-Apple.jpeg",
                size: "thumb",
            },
        ],
        source: "benzinga",
        summary: "Apple stock could come away a winner after Friday&#39;s quarterly rebalancing for index funds.",
        symbols: [
            "AAPL",
            "MSFT",
            "SPY",
        ],
        updated_at: "2024-09-20T13:48:40Z",
        url: "https://www.benzinga.com/news/24/09/40958207/how-apple-stock-could-gain-from-fridays-s-p-500-rebalancing-what-investors-should-know",
    },
]
```

</details>

## Contribution

If you would like to contribute to the project, PR's are welcome. The Broker API is primarily where the help would be needed.

## License

This project is licensed under the MIT and APACHE License.
