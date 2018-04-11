use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};
use regex::Regex;

// TODO: create a post struct to hold the title, image, date, number of liked, link to original
// post

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

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn parse_9gag_test() {
        assert_eq!(2, 2);
    }
}
