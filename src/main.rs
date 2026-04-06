//`?` only works if your function returns a `Result` or `Option`

mod api;
mod app;
mod ui;
use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(); //canot write let in global level
    App::run(&mut app).await?;

    Ok(())
}
//let app = App::new();    //canot write let in global level
