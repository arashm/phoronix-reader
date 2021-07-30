mod article;
use crate::article::Article;
mod linesplit;

use ansi_term::Colour::{Cyan, Green, Yellow};
use ansi_term::Style;

#[tokio::main]
async fn main() {
    fetch_articles().await;
}

async fn fetch_articles() -> () {
    for article in phoronix_articles().await.iter().rev() {
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

async fn phoronix_articles() -> Vec<Article> {
    Article::get_articles(&open_phoronix().await.unwrap())
}

async fn open_phoronix() -> Result<String, reqwest::Error> {
    Ok(reqwest::get("https://www.phoronix.com/scan.php?page=home")
        .await?
        .text()
        .await?)
}
