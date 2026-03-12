
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Comment {
    pub id: u64,
    pub by: Option<String>,
    pub text: Option<String>,
    pub kids: Option<Vec<u64>>,
    pub time: Option<u64>,
    pub deleted: Option<bool>,
    pub dead: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Story{
    pub by: Option<String>, // this part might be empty so to handle NULL-> Option
    pub descendants: Option<i32>,  
    pub id: i32,
    pub kids: Option<Vec<i32>>,
    pub score: Option<i32>,
    pub text: Option<String>,
    pub time: Option<i32>,
    pub title: String,
    pub r#type: Option<String>,
    pub url: Option<String>
}

use std::time::{SystemTime, UNIX_EPOCH};
pub fn time_ago(unix_timestamp: i32) -> String {
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


//Why a struct instead of free functions? Because reqwest::Client manages a connection pool internally — you want to create it once and reuse it, not make a new one per request
//If something is expensive to create and safe to reuse — create it once, wrap it in a struct, pass the struct around
#[derive(Debug, Clone)]
pub struct HnClient{             //learned::wrap external resources in a struct, expose behavior through methods.
    client: reqwest::Client       // the internal connection pool lives here  as the struct owns the client
}


impl HnClient{
    pub fn new() -> Self{               
        Self{
            client: reqwest::Client::new()   // created a new client using the constructor   
        }
    }
    
    //.send() => if making a single req in a script we dont require it .
    // When you use a persistent reqwest::Client, calling .get() only returns a RequestBuilder. This allows you to chain more settings (like headers or query parameters) before finally triggering the network call with .send()
 
   pub async fn fetch_top_stories(&self) -> Result<Vec<i64>, Box<dyn std::error::Error>>{ //either vector of ids or error
       let ids: Vec<i64> = self.client.get("https://hacker-news.firebaseio.com/v0/topstories.json").send().await?.json().await?;
       Ok(ids)
   }
   
   pub async fn fetch_story(&self, id: i64) -> Result<Story, Box<dyn std::error::Error>>{
       let story:Story = self.client.get(format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id)).send().await?.json().await?;
       Ok(story)
   }
      
    
}




