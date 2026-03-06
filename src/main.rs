
//`?` only works if your function returns a `Result`

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
    let json_body: Vec<u64>= reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").await?.json().await?;
    println!("string body :{:?}", json_body);
    println!("!st 10 ids:{:?}",&json_body[..10]);
    println!("!st 10 ids:{:?}",&json_body[..10].to_vec());

    Ok(())
}