mod browser_utils;
use browser_utils::{BrowserUtils, TabUtils};
use scraper::{Html, Selector};
use thiserror::Error;
use tokio::time::{sleep, Duration};
use headless_chrome::Browser;

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
pub async fn is_episode_out(
    id: &str,
    episode: u32,
    maybe_browser: Option<Browser>,
) -> Result<bool, IsEpisodeOutError> {
    let url = format!("https://9anime.to/watch/{id}/ep-{episode}");
    let (_browser, tab) = BrowserUtils::create_browser_tab(maybe_browser)
        .map_err(|e| IsEpisodeOutError::CreateBrowserTabError(e))?;

    // element we use to determine whether page has loaded or not
    let load_selector = format!("#watch-main[data-url|=\"https://9anime.to/watch/{id}\"]");

    // first navigation to check whether episode is out or not
    tab.navigate_to(&url)
        .map_err(|e| IsEpisodeOutError::TabNavigateError(e.to_string()))?
        .wait_until_navigated()
        .map_err(|e| IsEpisodeOutError::TabNavigateError(e.to_string()))?
        .wait_for_element(&load_selector)
        .map_err(|e| IsEpisodeOutError::TabNavigateError(e.to_string()))?;

    // buffer to ensure redirect has time to update url
    sleep(Duration::from_millis(500)).await;

    let is_out = tab.get_url() == url;

    // wait 2 seconds before trying confirmation to add buffer
    sleep(Duration::from_millis(2000)).await;

    // second navigation so we can confirm whether episode is out or not
    tab.navigate_to(&url)
        .map_err(|e| IsEpisodeOutError::TabNavigateError(e.to_string()))?
        .wait_until_navigated()
        .map_err(|e| IsEpisodeOutError::TabNavigateError(e.to_string()))?
        .wait_for_element(&load_selector)
        .map_err(|e| IsEpisodeOutError::TabNavigateError(e.to_string()))?;

    // buffer to ensure redirect has time to update url
    sleep(Duration::from_millis(500)).await;

    // clean up the tab
    match tab.close(true) {
        Ok(_) => (),
        Err(err) => {
            println!("Error trying to close tab: {}", err.to_string());
        }
    }

    let is_out_confirmed = tab.get_url() == url;

    // if the two attempts do not agree we assume the episode is not out
    if is_out != is_out_confirmed {
        return Ok(false);
    }

    Ok(is_out)
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
    let (_browser, tab) = BrowserUtils::create_browser_tab(None)
        .map_err(|e| ScrapeAnimeError::CreateBrowserTabError(e))?;

    // element we use to determine whether page has loaded or not
    let load_selector = format!("#watch-main[data-url|=\"https://9anime.to/watch/{id}\"]");

    tab.navigate_to(&url)
        .map_err(|e| ScrapeAnimeError::TabNavigateError(e.to_string()))?
        .wait_until_navigated()
        .map_err(|e| ScrapeAnimeError::TabNavigateError(e.to_string()))?
        .wait_for_element(&load_selector)
        .map_err(|e| ScrapeAnimeError::TabNavigateError(e.to_string()))?;

    // small buffer to make sure page loaded
    sleep(Duration::from_millis(1000)).await;

    let content = tab
        .get_content()
        .map_err(|e| ScrapeAnimeError::ContentRetrievalError(e.to_string()))?;

    // clean up the tab
    match tab.close(true) {
        Ok(_) => (),
        Err(err) => {
            println!("Error trying to close tab: {}", err.to_string());
        }
    }

    let document = Html::parse_document(&content);

    let name = Selector::parse("div.info h1.title")
        .map_err(|e| ScrapeAnimeError::SelectorCreationError(e.to_string()))?;
    let name = document
        .select(&name)
        .next()
        .ok_or(ScrapeAnimeError::SelectorNotFoundError(String::from(
            "Name not found.",
        )))?;
    let name = name.text().collect::<Vec<_>>()[0];

    let desc = Selector::parse("div.info div.shorting div.content")
        .map_err(|e| ScrapeAnimeError::SelectorCreationError(e.to_string()))?;
    let desc = document
        .select(&desc)
        .next()
        .ok_or(ScrapeAnimeError::SelectorNotFoundError(String::from(
            "Description not found.",
        )))?;
    let desc = desc.text().collect::<Vec<_>>()[0];

    let poster_img = Selector::parse("div.binfo div.poster span img")
        .map_err(|e| ScrapeAnimeError::SelectorCreationError(e.to_string()))?;
    let poster_img =
        document
            .select(&poster_img)
            .next()
            .ok_or(ScrapeAnimeError::SelectorNotFoundError(String::from(
                "Poster image not found.",
            )))?;
    let poster_img =
        poster_img
            .value()
            .attr("src")
            .ok_or(ScrapeAnimeError::SelectorNotFoundError(String::from(
                "Poster image src attribute not found.",
            )))?;

    let total_episodes = Selector::parse("div.info div.bmeta div.meta div")
        .map_err(|e| ScrapeAnimeError::SelectorCreationError(e.to_string()))?;
    let total_episodes = document.select(&&total_episodes).collect::<Vec<_>>();
    let mut total_episodes_count: u32 = 12;

    for ep in total_episodes {
        let text = ep.text().collect::<Vec<_>>();
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
