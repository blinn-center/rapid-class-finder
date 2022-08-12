use eyre::Result;
use log::{debug, trace};

use crate::data::{
    db::{create_empty_database, insert_all_courses},
    types::Everything,
};

use self::db::Database;

pub mod db;
pub mod queries;
pub mod types;

pub async fn download_everything(url: &str) -> Result<Everything> {
    trace!("downloading everything.json from {:#?}", url);
    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn download_full_database(url: &str) -> Result<Database> {
    debug!("reading everything");
    let everything = download_everything(url).await?;
    debug!("creating database");
    let mut db = create_empty_database()?;
    debug!("inserting courses");
    let inserted_count = insert_all_courses(&mut db, &everything)?;
    debug!("inserted {} courses", inserted_count);
    Ok(db)
}
