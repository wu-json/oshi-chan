use reqwest::{Client};

use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Page;

fn main() {
    let url: &str = "https://9anime.to/watch/mashle-magic-and-muscles.7j2zj/ep-100";

    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();

    tab.navigate_to(url).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(500));

    let res = tab.get_url();
    println!("URL: {}", res);

    let res = tab.get_content().unwrap();
    println!("TITLE: {}", res);

}