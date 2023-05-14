#[tokio::main]
async fn main() {
   // let is_out: bool = scrape_9anime::is_episode_out("mashle-magic-and-muscles.7j2zj", 81).await;
   // println!("IS OUT: {}", is_out);

   let res = scrape_9anime::scrape_anime("mashle-magic-and-muscles.7j2zj").await;
   println!("RESULT: {:?}", res);
}