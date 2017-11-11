extern crate hyper;
extern crate tokio_core;
extern crate futures;
extern crate select;
extern crate hyper_tls;
extern crate ansi_term;

mod article;
use article::Article;
mod linesplit;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future;
use hyper::{Client, Error};
use hyper_tls::HttpsConnector;
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
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let uri = "https://www.phoronix.com/scan.php?page=home".parse().unwrap();
    let request = client.get(uri).and_then(|res| {
        println!("Response status: {}", res.status());
        res.body().fold(Vec::new(), |mut v, chunk| {
            v.extend(&chunk[..]);
            future::ok::<_, Error>(v)
        }).and_then(|chunks| {
            let s = String::from_utf8(chunks).unwrap();
            future::ok::<_, Error>(s)
        })
    });
    core.run(request).unwrap()
}
