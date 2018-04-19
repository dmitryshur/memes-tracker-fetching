use fetching::parse_9gag::parse_9gag_posts;

#[derive(Debug)]
pub struct Post {
    id: String,
    // TODO: change to a URL type from some crate
    url: String,
    title: String,
    post_type: String,
    vote_count: u32,
    images: Vec<PostImage>
}

#[derive(Debug)]
pub struct PostImage {
    name: String,
    width: u32,
    height: u32,
    // TODO: change to a URL type from some crate
    url: String
}

pub fn parse_posts(html: &Vec<String>) {
    for (index, text) in html.iter().enumerate() {
        match index {
            // 9gag
            0 => {
                println!("9gag body");
                parse_9gag_posts(&text);
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

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn it_workes() {
        assert_eq!(1, 1);
    }
}
