use mongodb::{Client, Collection, Database};

pub mod schemas;

pub struct DB {
    client: Client,
    db: Database
}

impl DB {
    pub async fn new(uri: &str) -> Result<DB, mongodb::error::Error> {
        let client = Client::with_uri_str(uri).await?;
        let db = client.database("pielegniarki");

        Ok(DB {
            client,
            db
        })
    }

    pub fn collections(&self) -> CollectionSelector {
        CollectionSelector { db: &self.db }
    }
}

pub struct CollectionSelector<'a> {
    db: &'a Database
}

impl<'a> CollectionSelector<'a> {
    pub fn doctor(&self) -> Collection<schemas::doctor::Doctor> {
        self.db.collection("doctor")
    }
}