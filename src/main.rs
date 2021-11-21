mod theme;
use std::env;
use std::error::Error;

use clap::{App, Arg};
use dotenv::dotenv;

use newsapi::{get_articles, Articles};

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top Headlines\n\n");
    for i in &articles.articles {
        theme.print_text(&format!("`{}`", i.title));
        theme.print_text(&format!("> *{}*", i.url));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Quickly")
        .version("0.1.0")
        .author("Luis C. <luis@luiscardoso.dev>")
        .about("Quickly is a simple CLI that lets you search for headlines news.")
        .arg(
            Arg::new("country")
                .index(1)
                .possible_values(["br", "fr", "gb", "us"])
                .value_name("COUNTRY")
                .about("The 2-letter ISO 3166-1 code of the country you want to get headlines for.")
                .takes_value(true)
                .default_value("us"),
        )
        .arg(
            Arg::new("category")
                .index(2)
                .possible_values([
                    "business",
                    "entertainment",
                    "general",
                    "health",
                    "science",
                    "sports",
                    "technology",
                ])
                .value_name("CATEGORY")
                .about("The category you want to get headlines for.")
                .takes_value(true)
                .default_value("technology"),
        )
        .arg(
            Arg::new("query")
                .index(3)
                .value_name("QUERY")
                .about("Keywords or a phrase to search for.")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    dotenv().ok();

    let api_key = env::var("API_KEY")?;
    let country = matches.value_of("country").unwrap();
    let category = matches.value_of("category").unwrap();

    if matches.is_present("query") {
        let query = matches.value_of("query").unwrap();
        let url = format!(
            "https://newsapi.org/v2/top-headlines?country={0}&category={1}&q={2}&apiKey={3}",
            country, category, query, api_key
        );
        let articles = get_articles(&url)?;

        render_articles(&articles);

        Ok(())
    } else {
        let url = format!(
            "https://newsapi.org/v2/top-headlines?country={0}&category={1}&apiKey={2}",
            country, category, api_key
        );
        let articles = get_articles(&url)?;

        render_articles(&articles);

        Ok(())
    }
}
