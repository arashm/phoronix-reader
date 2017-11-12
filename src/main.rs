extern crate select;
extern crate ansi_term;
extern crate reqwest;

mod article;
use article::Article;
mod linesplit;

use std::io::Read;
use ansi_term::Style;
use ansi_term::Colour::{Green, Cyan, Yellow};

fn main() {
    let phoronix_articles = Article::get_articles(open_phoronix().as_str());
    for article in phoronix_articles.iter().rev() {
        let summary = linesplit::split_by_chars(&article.summary, 77).join("\n\t");
        println!(
            "* {}\n\t{}\n\t{}\n\t{}",
            Style::new().bold().fg(Green).paint(article.title.as_str()),
            Style::new().fg(Cyan).paint(article.link.as_str()),
            Style::new().fg(Yellow).paint(article.details.as_str()),
            Style::new().bold().paint(summary.as_str())
        );
    }
}

fn open_phoronix() -> String {
    let mut response = reqwest::get("https://www.phoronix.com/scan.php?page=home").unwrap();
    let mut content = String::new();
    response.read_to_string(&mut content).unwrap();
    content
}
