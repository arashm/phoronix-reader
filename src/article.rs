use select::document::Document;
use select::predicate::{Class, Name};
use select::node::Node;

pub struct Article {
    pub title: String,
    pub link: String,
    pub details: String,
    pub summary: String,
}

impl Article {
    fn new(node: &Node) -> Article {
        let header = node.find(Name("a")).nth(0).unwrap();
        let mut link = String::from(header.attr("href").unwrap());
        if !link.is_empty() { link = String::from("https://www.phoronix.com") + &link }
        let mut details = node.find(Class("details")).nth(0).unwrap().text();
        if details.contains("Add A Comment") {
            details = details.replace("Add A Comment", "0 Comments");
        }
        let summary = node.find(Name("p")).nth(0).unwrap().text();
        Article{ title: header.text(), link: link, details: details, summary: summary }
    }

    pub fn get_articles(html: &str) -> Vec<Article> {
        Document::from(html) // Open the HTML document
            .find(Name("article")) // Make an Iterator over <article> nodes
            .map(|node| Article::new(&node)) // Map each article to an Article struct
            .collect() // return it as a Vec<Article>
    }
}
