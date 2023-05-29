mod browser_utils;

use browser_utils::{BrowserUtils, TabUtils};
use headless_chrome::Browser;
use scraper::{Html, Selector};
use thiserror::Error;
use tokio::time::{sleep, Duration};

#[derive(Error, Debug)]
pub enum IsEpisodeOutError {
    #[error("Create browser tab error.")]
    CreateBrowserTabError(browser_utils::CreateBrowserTabError),
    #[error("Tab navigate error.")]
    TabNavigateError(String),
}

/// Checks whether an episode is out by navigating to the url of the episode.
/// 9anime will redirect the client to episode 1 url if the episode is not out,
/// otherwise it will render the episode page.
pub async fn is_episode_out(id: &str, episode: u32) -> Result<bool, IsEpisodeOutError> {
    let url = format!("https://9anime.to/watch/{id}/ep-{episode}");
    let (_browser, tab) = BrowserUtils::create_browser_tab()
        .map_err(|e| IsEpisodeOutError::CreateBrowserTabError(e))?;

    tab.navigate_to(&url)
        .map_err(|e| IsEpisodeOutError::TabNavigateError(e.to_string()))?
        .wait_until_navigated()
        .map_err(|e| IsEpisodeOutError::TabNavigateError(e.to_string()))?;

    // small buffer to be extra sure redirect occurred if episode is not out
    sleep(Duration::from_millis(1000)).await;

    Ok(tab.get_url() == url)
}

#[derive(Debug)]
pub struct Anime {
    pub id: String,
    pub name: String,
    pub description: String,
    pub poster_img_url: String,
    pub total_episodes: u32,
}

#[derive(Error, Debug)]

pub enum ScrapeAnimeError {
    #[error("Create browser tab error.")]
    CreateBrowserTabError(browser_utils::CreateBrowserTabError),
    #[error("Tab navigate error.")]
    TabNavigateError(String),
    #[error("Content retrieval error.")]
    ContentRetrievalError(String),
    #[error("Selector creation error.")]
    SelectorCreationError(String),
    #[error("Selector not found error.")]
    SelectorNotFoundError(String),
}

pub async fn scrape_anime(id: &str) -> Result<Anime, ScrapeAnimeError> {
    let url = format!("https://9anime.to/watch/{id}");
    let (_browser, tab) = BrowserUtils::create_browser_tab()
        .map_err(|e| ScrapeAnimeError::CreateBrowserTabError(e))?;

    tab.navigate_to(&url)
        .map_err(|e| ScrapeAnimeError::TabNavigateError(e.to_string()))?
        .wait_until_navigated()
        .map_err(|e| ScrapeAnimeError::TabNavigateError(e.to_string()))?;

    // small buffer to make sure page loaded
    sleep(Duration::from_millis(1000)).await;

    let content: String = match tab.get_content() {
        Ok(c) => c,
        Err(e) => return Err(ScrapeAnimeError::ContentRetrievalError(e.to_string())),
    };

    let document: Html = Html::parse_document(&content);

    let name: Selector = match Selector::parse("div.info h1.title") {
        Ok(s) => s,
        Err(e) => return Err(ScrapeAnimeError::SelectorCreationError(e.to_string())),
    };
    let name: scraper::ElementRef = match document.select(&name).next() {
        Some(s) => s,
        None => {
            return Err(ScrapeAnimeError::SelectorNotFoundError(String::from(
                "Name not found.",
            )))
        }
    };
    let name: Vec<&str> = name.text().collect::<Vec<_>>();
    let name: &str = name[0];

    let desc: Selector = match Selector::parse("div.info div.shorting div.content") {
        Ok(s) => s,
        Err(e) => return Err(ScrapeAnimeError::SelectorCreationError(e.to_string())),
    };
    let desc: scraper::ElementRef = match document.select(&desc).next() {
        Some(s) => s,
        None => {
            return Err(ScrapeAnimeError::SelectorNotFoundError(String::from(
                "Description not found.",
            )))
        }
    };
    let desc: Vec<&str> = desc.text().collect::<Vec<_>>();
    let desc: &str = desc[0];

    let poster_img: Selector = match Selector::parse("div.binfo div.poster span img") {
        Ok(s) => s,
        Err(e) => return Err(ScrapeAnimeError::SelectorCreationError(e.to_string())),
    };
    let poster_img: scraper::ElementRef = match document.select(&poster_img).next() {
        Some(s) => s,
        None => {
            return Err(ScrapeAnimeError::SelectorNotFoundError(String::from(
                "Poster image not found.",
            )))
        }
    };
    let poster_img: &str = match poster_img.value().attr("src") {
        Some(s) => s,
        None => {
            return Err(ScrapeAnimeError::SelectorNotFoundError(String::from(
                "Poster image src attribute not found.",
            )))
        }
    };

    let total_episodes: Selector = match Selector::parse("div.info div.bmeta div.meta div") {
        Ok(s) => s,
        Err(e) => return Err(ScrapeAnimeError::SelectorCreationError(e.to_string())),
    };
    let total_episodes: Vec<scraper::ElementRef> =
        document.select(&&total_episodes).collect::<Vec<_>>();
    let mut total_episodes_count: u32 = 12;

    for ep in total_episodes {
        let text: Vec<&str> = ep.text().collect::<Vec<_>>();
        if text[0] == "Episodes: " {
            total_episodes_count = match text[1].parse::<u32>() {
                Ok(c) => c,
                Err(_) => {
                    println!(
                        "Error parsing total episode count. Falling back on default ({}).",
                        total_episodes_count
                    );
                    total_episodes_count
                }
            };
            break;
        }
    }

    Ok(Anime {
        id: String::from(id),
        name: String::from(name),
        description: String::from(desc),
        poster_img_url: String::from(poster_img),
        total_episodes: total_episodes_count,
    })
}
