use reqwest::{Response, get};

/// Fetch the pages of the meme sites and returns a vector of the responses
pub fn fetch_pages(links: Vec<&str>) -> Vec<String> {
    let mut responses = vec![];

    for link in links {
        let mut response = get(link).unwrap();
        responses.push(response.text().unwrap());
    }
    responses
}

#[cfg(test)]
mod tests {
    use reqwest::get;

    #[test]
    fn fetch_pages() {
        let links = vec![
            "https://9gag.com/hot",
            "https://www.memecenter.com",
            "https://www.memedroid.com/",
            "http://www.quickmeme.com",
        ];
        for link in links {
            let mut response = get(link).unwrap();
            assert_eq!(response.status().is_success(), true);
        }
    }
}
