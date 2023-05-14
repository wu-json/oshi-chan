use headless_chrome::Browser;
use tokio::time::{sleep, Duration};
use scraper::{Html,Selector};

// This code tries to see whether an episode is out by navigating to the
// url of the episode. 9anime will redirect the client to episode 1 url
// if the episode is not out.
pub async fn is_episode_out(id: &str, episode: u32) -> bool {
    let url: String = format!("https://9anime.to/watch/{id}/ep-{episode}");

    let browser: Browser = Browser::default().unwrap();
    let tab: std::sync::Arc<headless_chrome::Tab> = browser.new_tab().unwrap();

    tab.enable_stealth_mode().unwrap();
    tab.set_user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36", Some("en-US,en;q=0.9,hi;q=0.8,es;q=0.7,lt;q=0.6"), Some("macOS")).unwrap();

    tab.navigate_to(&url).unwrap();

    sleep(Duration::from_millis(3000)).await;

    let new_url: String = tab.get_url();

    new_url == url
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
    let name_selector:Selector = Selector::parse("div.info h1.title").unwrap();

    let name_elem = document.select(&name_selector).next().unwrap();
    let name: Vec<&str> = name_elem.text().collect::<Vec<_>>();





    Anime {
        id: String::from(""),
        name: String::from(name[0]),
        description: String::from(""),
        poster_img_url: String::from(""),
        total_episodes: 23
    }
}