extern crate reqwest;
extern crate select;

mod fetching;

use fetching::fetch::fetch_pages;
use fetching::parse::parse_bodies;

fn main() {
    let links = vec![
        "https://9gag.com",
        "https://www.memecenter.com",
        "https://www.memedroid.com/",
        "http://www.quickmeme.com",
    ];
    let pages_bodies = fetch_pages(links);
    parse_bodies(&pages_bodies);
}
