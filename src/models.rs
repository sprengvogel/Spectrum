use chrono::NaiveDateTime;

use super::schema::fileentries;

#[derive(Queryable)]
pub struct FileEntry {
    pub id: i32,
    pub filename: String,
    pub file_last_modified: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "fileentries"]
pub struct NewFileEntry<'a> {
    pub filename: &'a str,
    pub file_last_modified: &'a NaiveDateTime,
}
