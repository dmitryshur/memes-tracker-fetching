use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};
use regex::Regex;

#[derive(Debug)]
struct Post {
    id: String,
    // later change to a URL type from some crate
    url: String,
    title: String,
    post_type: String,
    vote_count: u32,
    images: Vec<PostImage>
}

#[derive(Debug)]
struct PostImage {
    name: String,
    width: u32,
    height: u32,
    // later change to a URL type from some crate
    url: String
}

pub fn parse_bodies(bodies: &Vec<String>) {
    for (index, body) in bodies.iter().enumerate() {
        match index {
            // 9gag
            0 => {
                println!("9gag body");
                parse_9gag_body(&body);
            }

            // memecenter
            1 => {
                println!("memecenter body");
            }

            // memdroid
            2 => {
                println!("memdroid bodies");
            }

            // quickmeme
            3 => {
                println!("quickmeme bodies");
            }

            _ => (),
        }
    }
}

fn parse_9gag_body(page: &str) {
    let document = Document::from(page);

    // the script element which contains all the data and needs to be parsed
    let element = document
        .find(Name("script"))
        .max_by(|first, second| first.text().len().cmp(&second.text().len()))
        .unwrap()
        .text();
}

// later will return Vec<Post>
fn parse_9gag_posts(body: &str) {
    let reg = Regex::new(r#""id":"(.+?)""#).unwrap();
    for group in reg.captures_iter(body) {
        println!("{}", &group[1]);
    }
}

// TODO: Add tests for parsing the posts
#[cfg(test)]
mod tests {
    use ::*;


    #[test]
    fn parse_9gag_test() {
        let text = r#"{"id":"ajEg2Bq","url":"http:\/\/9gag.com\/gag\/ajEg2Bq","title":"Boss told me to make an ethernet cable today. No specification of length. Jokes on them.","type":"Photo","nsfw":0,"upVoteCount":2787,"hasLongPostCover":0,"images":{"image700":{"width":700,"height":510,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_700b.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_700bwp.webp"},"image460":{"width":460,"height":335,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_460s.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_460swp.webp"}},"sourceDomain":"","sourceUrl":"","commentsCount":124,"tags":[],"descriptionHtml":""},{"id":"aEx8LWN","url":"http:\/\/9gag.com\/gag\/aEx8LWN","title":"They took our jobs :D","type":"Photo","nsfw":0,"upVoteCount":3311,"hasLongPostCover":0,"images":{"image700":{"width":660,"height":993,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_700b.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_700bwp.webp"},"image460":{"width":460,"height":692,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_460s.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_460swp.webp"}},"sourceDomain":"","sourceUrl":"","commentsCount":137,"tags":[],"descriptionHtml":""}"#;
        assert_eq!(2, 2);
    }
}
