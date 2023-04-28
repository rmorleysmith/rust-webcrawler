use std::collections::HashSet;
use scraper::{Html, Selector};
use url::Url;

fn main() {
    crawl_from_seed("https://www.bbc.co.uk");
}

fn crawl_from_seed(seed: &str) {
    // Add the seed URL to the crawl queue
    let mut crawl_queue = HashSet::new();
    crawl_queue.insert(Url::parse(seed).unwrap());

    // Keep track of crawled URLs
    let mut crawled_urls = HashSet::new();

    while !crawl_queue.is_empty() {
        // Get the next URL to crawl
        let url_to_crawl = crawl_queue.iter().next().unwrap().clone();

        // Remove the URL from the queue
        crawl_queue.remove(&url_to_crawl);

        // Check if we crawled this URL yet
        if crawled_urls.contains(&url_to_crawl) {
            continue;
        }

        // Fetch the URL and print the response
        let response = reqwest::blocking::get(url_to_crawl.as_str()).unwrap();
        let body = response.text().unwrap();
        // Add any links found to the set of URLs to crawl
        let new_urls = extract_links(&body);
        for new_url in new_urls {
            if !crawled_urls.contains(&new_url) {
                crawl_queue.insert(new_url);
            }
        }
        
        // Print out the URL we just crawled
        println!("Crawled: {}", &url_to_crawl);
        
        // Add this URL to the crawled list
        crawled_urls.insert(url_to_crawl);
    }
}

fn extract_links(html: &str) -> HashSet<Url> {
    // Create a HashSet to store the links
    let mut links = HashSet::new();
    
    // Parse the HTML string into a scraper document
    let doc = Html::parse_document(html);

    // Define a CSS selector to match all <a> tags with an "href" attribute
    let selector = Selector::parse("a[href]").unwrap();

    // Extract the links from appropriate <a> tags
    for link in doc.select(&selector) {
        let href = link.value().attr("href").unwrap();

        // Parse this href into a URL
        if let Ok(url) = Url::parse(href) {
            links.insert(url);
        }
    }

    return links;
}