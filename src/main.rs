use app::App;

mod app;
mod db;
mod http_client;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    // let db = db::DB::new("mongodb://padmin:zaq1@localhost:27017").await?;
    let db = db::DB::new("mongodb://localhost:27017").await?;

    let client = http_client::HttpClient::new();
    
    App::serve(db, client).await?;

    Ok(())
}