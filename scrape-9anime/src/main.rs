#[tokio::main]
async fn main() {
   let is_out = scrape_9anime::is_episode_out("mashle-magic-and-muscles.7j2zj", 332).await;
   println!("IS OUT: {}", is_out);
}