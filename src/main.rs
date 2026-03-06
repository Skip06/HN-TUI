
//`?` only works if your function returns a `Result`

// use serde::Deserialize;
#[tokio::main]
// #[derive(Debug,Deserialize)]
async fn main()->Result<(),Box<dyn std::error::Error>>{
    let body = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").await?;
    println!("string body :{:?}", body.text().await?);
    // println!("json type body:{}", body.json().await?);   //cant cause derive(Deserialize) is only on structs, enums
    Ok(())
}