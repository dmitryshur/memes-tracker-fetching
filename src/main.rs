extern crate reqwest;

mod fetching;

use fetching::fetch::fetch_pages;

fn main() {
    let links = vec![
        "https://9gag.com",
        "https://www.memecenter.com",
        "https://www.memedroid.com/",
        "http://www.quickmeme.com",
    ];
    println!("{:?}", fetch_pages(links));
}
