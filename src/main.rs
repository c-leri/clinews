mod theme;

use std::error::Error;
use newsapi::{Article, Country, Endpoint, NewsAPI};
use dotenv::dotenv;

fn render_articles(articles: &Vec<Article>)
{
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for article in articles
    {
        theme.print_text(&format!("`{}`", article.title()));
        theme.print_text(&format!("> *{}*", article.url()));
        theme.print_text("---")
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
    dotenv();

    let api_key = std::env::var("API_KEY")?;

    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines);
    newsapi.country(Country::FR);

    let newsapi_reponse = newsapi.fetch()?;

    render_articles(&newsapi_reponse.articles());

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>>
// {
//     dotenv();
//
//     let api_key = std::env::var("API_KEY")?;
//
//     let mut newsapi = NewsAPI::new(&api_key);
//     newsapi.endpoint(Endpoint::TopHeadlines);
//     newsapi.country(Country::FR);
//
//     let newsapi_reponse = newsapi.fetch_async().await?;
//
//     render_articles(&newsapi_reponse.articles());
//
//     Ok(())
// }