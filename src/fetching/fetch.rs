use reqwest::{Response, get};

/// Fetch the pages of the meme sites and returns a vector of the responses
pub fn fetch_pages(links: Vec<&str>) -> Vec<Response> {
    let mut responses = vec![];

    for link in links {
        let mut response = get(link).unwrap();
        responses.push(response);
    }
    responses
}
