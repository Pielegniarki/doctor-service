use app::App;

mod app;
mod db;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let db = db::DB::new("mongodb://pielegniarki:pielegniarki@localhost:27017").await?;

    let client = reqwest::Client::new();
    
    App::serve(db, client).await?;

    Ok(())
}