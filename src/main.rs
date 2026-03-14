//`?` only works if your function returns a `Result` or `Option`

mod app;
mod api;
mod ui;
use api::HnClient;
use api::Story;
use api::time_ago;
use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(); //canot write let in global level
    App::run(&mut app).await?;
    
    Ok(())  
}
//let app = App::new();    //canot write let in global level
