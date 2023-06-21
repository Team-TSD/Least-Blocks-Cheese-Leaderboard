use std::error::Error;

use scraper::{Html, Selector};

use crate::common::{CheeseRun, Country};
use crate::config::COUNTRY_MAP;

pub fn parse_profile(text: String) -> Result<(Option<Country>, Option<String>), anyhow::Error> {
    let document = Html::parse_document(&text);
    let img_selector = Selector::parse("img").unwrap();
    let mut country = None;
    let mut name = None;
    for img in document.select(&img_selector) {
        if let Some(class) = img.value().attr("class") {
            if class == "flag" {
                if let Some(country_code) = img.value().attr("alt") {
                    if let Some(c) = COUNTRY_MAP.get(country_code) {
                        country = Some(c.clone());
                    }
                }
            }
        }
    }
    let h1_selector = Selector::parse("h1").unwrap();
    for span in document.select(&h1_selector) {
        if let Some(class_name) = span.value().attr("class") {
            if class_name == "mainName"{
                name = Some(span.text().next().unwrap().trim().to_string());
            }
        }
    }
    Ok((country, name))
}

#[derive(Debug, Clone)]
struct HtmlMatchError {
    inner_html: String,
}

impl std::fmt::Display for HtmlMatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "html pattern can't be matched: {}", self.inner_html)
    }
}
impl Error for HtmlMatchError {}

pub fn parse_run_page(text: String) -> Result<(Vec<CheeseRun>, Option<f32>), anyhow::Error> {
    let mut runs: Vec<CheeseRun> = Vec::new();

    let document = Html::parse_document(&text);
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    for tr in document.select(&tr_selector).skip(1) {
        let mut t: u32 = 0;
        let mut pps: Option<f64> = None;
        let mut blocks: Option<u32> = None;
        let mut date: Option<String> = None;
        let mut link: Option<String> = None;
        for td in tr.select(&td_selector) {
            if t == 4 {
                pps = Some(td.inner_html().to_string().parse::<f64>()?);
            } else if t == 3 {
                blocks = Some(td.inner_html().to_string().parse::<u32>()?);
            } else if t == 6 {
                date = Some(td.inner_html().to_string());
            } else if t == 7 {
                let frag = Html::parse_fragment(&td.inner_html());
                if let Some(a) = frag.select(&a_selector).next() {
                    if let Some(href) = a.value().attr("href") {
                        link = Some(href.to_string());
                    }
                }
            }
            t += 1;
        }
        if let (Some(b), Some(d), Some(t)) = (blocks, date, pps) {
            let run = CheeseRun {
                blocks: b,
                date: d,
                replay: link,
                pps: t,
            };
            runs.push(run);
        } else {
            return Err(anyhow::Error::new(HtmlMatchError {
                inner_html: tr.inner_html(),
            }));
        }
    }
    let mut new_page = None;
    for btn in document.select(&a_selector) {
        if let Some(rel) = btn.value().attr("rel") {
            if rel == "next" {
                if let Some(href) = btn.value().attr("href") {
                    let url = reqwest::Url::parse(href)?;
                    if let Some(page) = url
                        .query_pairs()
                        .find(|(k, _v)| k == "page")
                        .map(|(_k, v)| v)
                    {
                        new_page = Some(page.to_string().parse::<f32>()?);
                    }
                }
            }
        }
    }
    Ok((runs, new_page))
}
