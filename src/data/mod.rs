use std::{fmt::Debug, fs::File, path::Path};

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

pub fn read_everything<P: AsRef<Path> + Debug>(path: P) -> Result<Everything> {
    trace!("reading everything.json from {:#?}", path);
    Ok(serde_json::from_reader(File::open(path)?)?)
}

pub fn get_full_database<P: AsRef<Path> + Debug>(path: P) -> Result<Database> {
    debug!("reading everything");
    let everything = read_everything(path)?;
    debug!("creating database");
    let mut db = create_empty_database()?;
    debug!("inserting courses");
    let inserted_count = insert_all_courses(&mut db, &everything)?;
    debug!("inserted {} courses", inserted_count);
    Ok(db)
}
