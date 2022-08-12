use eyre::Result;
use log::trace;
use rusqlite::{Connection, Transaction};
use tokio::sync::{Mutex, MutexGuard};

use crate::data::types::{Course, Everything};

pub fn create_empty_database() -> Result<Database> {
    const SCHEMA: &str = include_str!("sql/schema.sql");
    trace!("creating new in-memory database with schema: {}", SCHEMA);
    let conn = Connection::open_in_memory()?;
    conn.execute(SCHEMA, ())?;
    Ok(Database(Mutex::new(conn)))
}

pub fn insert_course(conn: &Transaction, course: &Course) -> Result<()> {
    conn.execute(
        include_str!("sql/insert_course.sql"),
        (
            &course.name,
            &course.method,
            &course.crn,
            &course.subject,
            &course.course_number,
            &course.section,
            &course.associated_term,
            &course.registration_start,
            &course.registration_end,
            &course.levels,
            &course.campus,
            &course.schedule_type,
            &course.full_method,
            &course.credits,
            &course.catalog_entry_link,
        ),
    )?;
    Ok(())
}

pub fn insert_all_courses(db: &mut Database, everything: &Everything) -> Result<usize> {
    trace!("bulk-inserting courses");
    let mut count = 0;
    let conn = db.0.get_mut();
    let tx = conn.transaction()?;
    for course in everything.course_iter() {
        count += 1;
        insert_course(&tx, course)?;
    }
    tx.commit()?;
    Ok(count)
}

pub struct Database(Mutex<Connection>);

impl Database {
    pub async fn conn(&self) -> MutexGuard<'_, Connection> {
        self.0.lock().await
    }
}
