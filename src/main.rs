mod theme;

use std::error::Error;
use newsapi::{Articles, get_articles};
use dotenv::dotenv;

fn render_articles(articles: &Articles)
{
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for article in &articles.articles
    {
        theme.print_text(&format!("`{}`", article.title));
        theme.print_text(&format!("> *{}*", article.url));
        theme.print_text("---")
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
    dotenv();

    let api_key = std::env::var("API_KEY")?;

    let url = format!("https://newsapi.org/v2/top-headlines?country=fr&apiKey={}", api_key);

    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}