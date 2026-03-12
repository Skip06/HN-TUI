//`?` only works if your function returns a `Result`

mod app;
mod api;
use api::Story;
use api::time_ago;
use api::HnClient;






#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
    let hn = HnClient::new(); //constructor called
    
    let top_ids = hn.fetch_top_stories().await?;// its a async funtion
    for id in &top_ids[..20]{   // we are borrowing then iterating
        let story = hn.fetch_story(*id).await?;
        println!("Titile: {}", story.title);
        println!("Score: {}", story.score.unwrap_or(0));
        println!("Time:{}", time_ago(story.time.unwrap_or(0)));
        println!("Author:{}", story.by.unwrap_or(String::from("not found")));
        println!("-----------------------------");
    }
    Ok(())
    
    
}

