extern crate regex;
extern crate reqwest;
extern crate select;
extern crate openssl_probe;

use std::env;
use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);
    let url = format!("https://www.youtube.com/channel/{}/videos", &args[1]);
    let re = match args.len() {
        3 => Regex::new(&args[2]).unwrap(),
        _ => Regex::new("").unwrap(),
    };

    let res = reqwest::get(&url).unwrap();
    assert!(res.status().is_success());

    Document::from_read(res)
        .unwrap()
        .find(Class("yt-lockup-title").descendant(Name("a")))
        .filter(|n| re.is_match(&n.text()))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("https://youtube.com{}", x));
}
