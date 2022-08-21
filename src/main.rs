use chrono::{DateTime, Local};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use spectrum::{create_file_entry, error::FileStoreError, establish_connection};
use std::{env, fs};

fn main() -> std::io::Result<()> {
    let pwd = &env::current_dir()?;
    let files = fs::read_dir(pwd)?;

    let conn = &establish_connection();
    for file_result in files {
        let file = file_result?;
        match store_file_in_db(file, conn) {
            Err(err) => {
                println!("Error {} ignored while storing files.", err);
                continue;
            }
            Ok(()) => continue,
        }
    }

    Ok(())
}

/*Stores the given file in the database. If this function returns Ok(), it is idempotent.
It will not insert or update files that are already in the db.
If this function returns Err, it will not have stored anything in the db, making it not idempotent. (It could succeed on a successive call).*/
fn store_file_in_db(
    file: fs::DirEntry,
    conn: &diesel::SqliteConnection,
) -> Result<(), FileStoreError> {
    use spectrum::schema::fileentries::dsl::*;

    let path = (&file).path();
    let file_name = path.to_str().expect("Filename was not valid utf-8");
    let last_modified = DateTime::<Local>::from((&file).metadata()?.created()?).naive_local();

    let already_exists: bool = fileentries
        .filter(filename.eq(file_name))
        .count()
        .first::<i64>(conn)?
        > 0;
    if !already_exists {
        create_file_entry(conn, file_name, &last_modified)?;
    }
    Ok(())
}
