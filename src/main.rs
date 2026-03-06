
//`?` only works if your function returns a `Result`
use serde::Deserialize;
    #[derive(Deserialize, Debug)]
struct Story{
    by: Option<String>, // this part might be empty so to handle NULL-> Option
    descendants: Option<i32>,  
    id: i32,
    kids: Option<Vec<i32>>,
    score: Option<i32>,
    text: Option<String>,
    time: Option<i32>,
    title: String,
    r#type: Option<String>,
    url: Option<String>
}
use std::time::{SystemTime, UNIX_EPOCH};

fn time_ago(unix_timestamp: i32) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32; //as i32 is typecasting
    
    let seconds_ago = now - unix_timestamp;
    
    if seconds_ago < 60 {
        format!("{}sec ago", seconds_ago)
    } else if seconds_ago < 3600 {
        format!("{}min ago", seconds_ago / 60)
    } else if seconds_ago < 86400 {
        format!("{}hrs ago", seconds_ago / 3600)
    } else if seconds_ago < 2592000 {
        format!("{}day ago", seconds_ago / 86400)
    } else {
        format!("{}month ago", seconds_ago / 2592000)
    }
}


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
    let body: Vec<i64>= reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").await?.json().await?;
    // println!("string body :{:?}", body);
    println!("!st 10 ids:{:?}",&body[..10]);
    println!("!st 10 ids:{:?}",&body[..10].to_vec());
    let id = 47191052;
    let s:Story = reqwest::get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id)).await?.json().await?;
    println!("story of {} is {:?}",id,s);
    for i in &body[..20]{
       let s1:Story =  reqwest::get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", i)).await?.json().await?;
       println!("Title:{}",s1.title);
       println!("Score:{}",s1.score.unwrap());
       println!("Time:{}",time_ago(s1.time.unwrap()));
       println!("Author:{}",s1.by.unwrap());
    }
    Ok(())
}