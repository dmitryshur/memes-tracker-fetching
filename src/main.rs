extern crate reqwest;
extern crate select;
extern crate regex;

mod fetching;

use fetching::fetch::fetch_pages;
use fetching::parse::parse_posts;

fn main() {
    let links = vec![
        "https://9gag.com",
        "https://www.memecenter.com",
        "https://www.memedroid.com/",
        "http://www.quickmeme.com",
    ];
    let pages_html = fetch_pages(links);
    parse_posts(&pages_html);
}
