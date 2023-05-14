use headless_chrome::Browser;
use scraper::{Html, Selector};
use thiserror::Error;
use tokio::time::{sleep, Duration};

#[derive(Error, Debug)]

pub enum IsEpisodeOutError {
    #[error("Browser error.")]
    BrowserError(String),
    #[error("Browser tab error.")]
    BrowserTabError(String),
    #[error("Stealth mode error.")]
    StealthModeError(String),
    #[error("Set user agent error.")]
    SetUserAgentError(String),
}

// This code tries to see whether an episode is out by navigating to the
// url of the episode. 9anime will redirect the client to episode 1 url
// if the episode is not out.
pub async fn is_episode_out(id: &str, episode: u32) -> Result<bool, IsEpisodeOutError> {
    let url: String = format!("https://9anime.to/watch/{id}/ep-{episode}");

    let browser: Browser = match Browser::default() {
        Ok(b) => b,
        Err(e) => return Err(IsEpisodeOutError::BrowserError(e.to_string())),
    };

    let tab: std::sync::Arc<headless_chrome::Tab> = match browser.new_tab() {
        Ok(t) => t,
        Err(e) => return Err(IsEpisodeOutError::BrowserTabError(e.to_string())),
    };

    match tab.enable_stealth_mode() {
        Ok(_) => {},
        Err(e) => return Err(IsEpisodeOutError::StealthModeError(e.to_string()))
    }

    match tab.set_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36", Some("en-US,en;q=0.9,hi;q=0.8,es;q=0.7,lt;q=0.6"), Some("macOS")) {
        Ok(_) => {},
        Err(e) => return Err(IsEpisodeOutError::SetUserAgentError(e.to_string()))
    }

    tab.navigate_to(&url).unwrap();

    sleep(Duration::from_millis(500)).await;

    let new_url: String = tab.get_url();

    Ok(new_url == url)
}

#[derive(Debug)]
pub struct Anime {
    pub id: String,
    pub name: String,
    pub description: String,
    pub poster_img_url: String,
    pub total_episodes: u32,
}

pub async fn scrape_anime(id: &str) -> Anime {
    let url: String = format!("https://9anime.to/watch/{id}");

    let browser: Browser = Browser::default().unwrap();
    let tab: std::sync::Arc<headless_chrome::Tab> = browser.new_tab().unwrap();

    tab.enable_stealth_mode().unwrap();
    tab.set_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36", Some("en-US,en;q=0.9,hi;q=0.8,es;q=0.7,lt;q=0.6"), Some("macOS")).unwrap();

    tab.navigate_to(&url).unwrap();

    sleep(Duration::from_millis(3000)).await;

    let content: String = tab.get_content().unwrap();
    let document: Html = Html::parse_document(&content);

    let name: Selector = Selector::parse("div.info h1.title").unwrap();
    let name: scraper::ElementRef = document.select(&name).next().unwrap();
    let name: Vec<&str> = name.text().collect::<Vec<_>>();
    let name: &str = name[0];

    let desc: Selector = Selector::parse("div.info div.shorting div.content").unwrap();
    let desc: scraper::ElementRef = document.select(&desc).next().unwrap();
    let desc: Vec<&str> = desc.text().collect::<Vec<_>>();
    let desc: &str = desc[0];

    let poster_img: Selector = Selector::parse("div.binfo div.poster span img").unwrap();
    let poster_img: scraper::ElementRef = document.select(&poster_img).next().unwrap();
    let poster_img: &str = poster_img.value().attr("src").unwrap();

    let total_episodes: Selector = Selector::parse("div.info div.bmeta div.meta div").unwrap();
    let total_episodes: Vec<scraper::ElementRef> =
        document.select(&&total_episodes).collect::<Vec<_>>();
    let mut total_episodes_count: u32 = 12;

    for ep in total_episodes {
        let text: Vec<&str> = ep.text().collect::<Vec<_>>();
        if text[0] == "Episodes: " {
            total_episodes_count = text[1].parse::<u32>().unwrap();
        }
    }

    Anime {
        id: String::from(id),
        name: String::from(name),
        description: String::from(desc),
        poster_img_url: String::from(poster_img),
        total_episodes: total_episodes_count,
    }
}
