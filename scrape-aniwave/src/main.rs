#[tokio::main]
async fn main() {
    let is_out: bool = scrape_aniwave::is_episode_out("mashle-magic-and-muscles.7j2zj", 3).await.unwrap();
    println!("IS OUT: {}", is_out);

    let path = scrape_aniwave::get_page_content("mashle-magic-and-muscles.7j2zj", 3)
        .await
        .unwrap();
    println!("Saved page contents to path: {}", path);

    let res = scrape_aniwave::scrape_anime("mashle-magic-and-muscles.7j2zj").await;
    println!("RESULT: {:?}", res.unwrap());
}
