use eyre::Result;
use fallible_iterator::FallibleIterator;
use log::trace;

use crate::data::{db::Database, types::SqlCourse};

impl Database {
    pub async fn course_count(&self) -> Result<usize> {
        const QUERY: &str = include_str!("sql/course_count.sql");
        trace!("executing course_count query: {}", QUERY);
        Ok(self.conn().await.query_row(QUERY, (), |row| row.get(0))?)
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<SqlCourse>> {
        let conn = self.conn().await;
        let mut stmt = conn.prepare(include_str!("sql/search.sql"))?;
        let rows = stmt.query((query, limit))?;
        Ok(rows
            .map(|row| {
                Ok(SqlCourse {
                    name: row.get(0)?,
                    method: row.get(1)?,
                    crn: row.get(2)?,
                    subject: row.get(3)?,
                    course_number: row.get(4)?,
                    section: row.get(5)?,
                    associated_term: row.get(6)?,
                    registration_start: row.get(7)?,
                    registration_end: row.get(8)?,
                    levels: row.get(9)?,
                    campus: row.get(10)?,
                    schedule_type: row.get(11)?,
                    full_method: row.get(12)?,
                    credits: row.get(13)?,
                    catalog_entry_link: row.get(14)?,
                })
            })
            .collect()?)
    }
}
