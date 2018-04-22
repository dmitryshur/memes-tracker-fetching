use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

use fetching::parse::Post;
use fetching::parse::PostImage;

//
//
// Public functions
//
//

pub fn parse_9gag_posts(text: &str) {
  let posts_text = parse_9gag_text(text);
  parse_9gag_post(&posts_text);
}

//
//
// Private functions
//
//

fn parse_9gag_text(text: &str) -> String {
  let document = Document::from(text);

  // the script element which contains all the data and needs to be parsed
  document
    .find(Name("script"))
    .max_by(|first, second| first.text().len().cmp(&second.text().len()))
    .unwrap()
    .text()
}

// TODO: return Vec<Post>
fn parse_9gag_post(text: &str) -> Vec<Post> {
  let mut posts = vec![];

  let reg = Regex::new(
    r#""id":"(.+?)","url":"(.+?)","title":"(.+?)","type":"(.+?)".+?"upVoteCount":(\d+),"hasLongPostCover":(\d),"images":\{(.+?})\}"#,
  ).expect("Couldn't parse the post data using regex");

  for group in reg.captures_iter(text) {
    posts.push(Post {
      id: group[1].to_string(),
      url: group[2].to_string(),
      title: group[3].to_string(),
      post_type: group[4].to_string(),
      vote_count: group[5].parse::<u32>().expect("Couldn't parse votes count to a number"),
      is_long: false,
      images: group[7].to_string(),
    });
  }

  posts
}

#[cfg(test)]
mod tests {
  use super::{parse_9gag_post, Post, PostImage};

  #[test]
  fn parse_9gag_post_test() {
    let text = r#"{"id":"ajEg2Bq","url":"http:\/\/9gag.com\/gag\/ajEg2Bq","title":"Boss told me to make an ethernet cable today. No specification of length. Jokes on them.","type":"Photo","nsfw":0,"upVoteCount":2787,"hasLongPostCover":0,"images":{"image700":{"width":700,"height":510,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_700b.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_700bwp.webp"},"image460":{"width":460,"height":335,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_460s.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_460swp.webp"}},"sourceDomain":"","sourceUrl":"","commentsCount":124,"tags":[],"descriptionHtml":""},{"id":"aEx8LWN","url":"http:\/\/9gag.com\/gag\/aEx8LWN","title":"They took our jobs :D","type":"Photo","nsfw":0,"upVoteCount":3311,"hasLongPostCover":0,"images":{"image700":{"width":660,"height":993,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_700b.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_700bwp.webp"},"image460":{"width":460,"height":692,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_460s.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_460swp.webp"}},"sourceDomain":"","sourceUrl":"","commentsCount":137,"tags":[],"descriptionHtml":""}"#;

    let parse_result = vec![
      Post {
        id: "ajEg2Bq".to_string(),
        url: r#"http:\/\/9gag.com\/gag\/ajEg2Bq"#.to_string(),
        title: "Boss told me to make an ethernet cable today. No specification of length. Jokes on them.".to_string(),
        post_type: "Photo".to_string(),
        vote_count: 2787,
        is_long: false,
        images: r#""image700":{"width":700,"height":510,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_700b.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_700bwp.webp"},"image460":{"width":460,"height":335,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_460s.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/ajEg2Bq_460swp.webp"}"#.to_string()
      },
      Post {
        id: "aEx8LWN".to_string(),
        url: r#"http:\/\/9gag.com\/gag\/aEx8LWN"#.to_string(),
        title: "They took our jobs :D".to_string(),
        post_type: "Photo".to_string(),
        vote_count: 3311,
        is_long: false,
        images: r#""image700":{"width":660,"height":993,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_700b.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_700bwp.webp"},"image460":{"width":460,"height":692,"url":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_460s.jpg","webpUrl":"https:\/\/img-9gag-fun.9cache.com\/photo\/aEx8LWN_460swp.webp"}"#.to_string()
      },
    ];
    assert_eq!(parse_9gag_post(text), parse_result);
  }
}


