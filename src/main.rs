use std::{collections::HashSet, sync::{Arc, Mutex}, thread};
use scraper::{Html, Selector};
use url::Url;
use crossbeam_channel::unbounded;

fn main() {
    crawl_from_seed("https://www.bbc.co.uk");
}

fn crawl_from_seed(seed: &str) {
    // Keep track of crawled URLs
    let crawled_urls = Arc::new(Mutex::new(HashSet::new()));

    // Create a channel so that threads can communicate with each other
    let (sender, receiver) = unbounded();

    // Use 16 threads
    for i in 0..16 {
        let sender = sender.clone();
        let receiver = receiver.clone();
        let crawled_urls = crawled_urls.clone();

        thread::spawn(move || loop {
            let url: Url = match receiver.recv() {
                Ok(url) => url,
                Err(_) => break,
            };

            println!("Thread {} received URL {}", i, url);

            // Fetch the URL
            let response = reqwest::blocking::get(url.as_str()).unwrap();
            let body = response.text().unwrap();

            // Add any links found to the set of URLs to crawl
            let new_urls = extract_links(&body);
            
            // Get crawled URLs with a lock 
            let mut crawled_urls = crawled_urls.lock().unwrap();

            // Insert the URL we just crawled
            crawled_urls.insert(url.clone());

            for new_url in new_urls {
                if !crawled_urls.contains(&new_url) {
                    match sender.send(new_url) {
                        Ok(_) => (),
                        Err(_) => println!("Error sending new URL to channel"),
                    }
                }
            }

            drop(crawled_urls);

             // Print out the URL we just crawled
            println!("Crawled: {}", url);
        });
    }

    match Url::parse(seed) {
        Ok(url) => { sender.send(url).unwrap() },
        Err(_) => (),
    }

    // Keep going until we stop
    loop {}
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