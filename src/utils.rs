//! Logic for the Translator (webscraping)
use scraper::{Html, Selector};
use reqwest::{StatusCode, Response};

use thiserror::Error;
use clap::Parser;

use cssparser::ParseError;
use selectors;

use colored::*;
use vecshard::{VecShard, ShardExt};

/// the clap info..
#[derive(Parser, Debug)]
#[clap(name = "Russian-English Translator")]
#[clap(author = "Gabriel Haeck")]
#[clap(version = "0.1.0")]
#[clap(about = "Translates a given russian word to english", long_about = None)]
pub struct ClapArgs {
    // original word to be translated
    #[clap(long, short)]
    pub word: String,
}

/// custom error
#[derive(Error, Debug)]
pub enum TranslError<'a> {
    // standard error...
    #[error("Reqwest error.")]
    ReqError {
        #[from]
        source: reqwest::Error,
    },
    // for Selector::parse() function.
    #[error("Could not retrieve selector.")]
    ParseError(ParseError<'a, selectors::parser::SelectorParseErrorKind<'a>>), // can't use #[from] here
    // if the word can't be found on the website
    #[error("Could not find the inputted word.")]
    FindError,
}

impl<'a> From<ParseError<'a, selectors::parser::SelectorParseErrorKind<'a>>> for TranslError<'a> {
    fn from(err: ParseError<'a, selectors::parser::SelectorParseErrorKind<'a>>) -> Self {
        TranslError::ParseError(err)
    }
}

// to have a cleaner function signature
type TResult<'a, T> = std::result::Result<T, TranslError<'a>>;

/// Given an str it will build the url.
fn build_url(string: &str) -> String {
    format!("https://en.openrussian.org/ru/{}", string)
}

/// The actual scrapping itself.
#[tokio::main]
pub async fn scrape() -> TResult<Vec<String>,'static> {
    let args: ClapArgs = ClapArgs::parse();

    let url = build_url(&args.word);

    let resp: Response = reqwest::get(url).await?; // able to reach the website

    let raw_html;
    if resp.status() == StatusCode::OK {
        raw_html = resp.text().await?;
    } else {
        return Err(TranslError::FindError);
    }

    let document = Html::parse_document(&raw_html);

    let li_selector = Selector::parse("p.tl")?;

    let mut transl_vec: Vec<String> = Vec::new();

    for node in document.select(&li_selector) {
        let word = node.inner_html().to_string();
        transl_vec.push(word);
    }

    Ok(transl_vec)
}

/// takes the vector of translated words and prints them.
pub fn print_vec(vec: Vec<String>) -> () {
    let (first, rest): (VecShard<String>, VecShard<String>) = vec.split_inplace_at(1);

    let first_string: Vec<String> = first.into();
    let rest_string: Vec<String> = rest.into();

    println!("{}{}", "Translation: ".green().bold(), first_string.get(0).unwrap());

    // print alternate translations if available
    if rest_string.is_empty() {
        println!("{}", "No alternative translations available.".cyan());
    } else {
        println!("{}{}", "Alternative(s): ".cyan(), rest_string.join(", "));
    }
}
