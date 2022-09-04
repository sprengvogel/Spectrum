use chrono::{DateTime, Local};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use spectrum::{create_file_entry, error::FileStoreError, establish_connection};
use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use std::{fs, io};

fn main() -> std::io::Result<()> {
    let pwd = Path::new("C:/Users/Niklas/Documents");
    let files = read_dir_recursively(pwd)?;

    let conn = &establish_connection();
    for file_entry in files {
        let file = match file_entry {
            Err(err) => {
                println!("Error {} ignored while reading files.", err);
                continue;
            }
            Ok(file) => file,
        };
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

fn read_dir_recursively(path: &Path) -> io::Result<Vec<io::Result<DirEntry>>> {
    if path.is_dir() {
        let read_dir: ReadDir = fs::read_dir(path)?;
        return Ok(do_recursive_read_and_collect(read_dir));
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "This function needs to be called with a directory",
    ))
}

fn do_recursive_read_and_collect(read_dir: ReadDir) -> Vec<io::Result<DirEntry>> {
    read_dir
        .map(|res| do_recursive_read(res))
        .flatten()
        .collect::<Vec<io::Result<DirEntry>>>()
}

fn do_recursive_read(dir_entry: io::Result<DirEntry>) -> Vec<io::Result<DirEntry>> {
    let path = match &dir_entry {
        Ok(entry) => entry.path(),
        Err(_) => return vec![dir_entry],
    };
    if path.is_dir() {
        let read_dir: ReadDir = match fs::read_dir(path) {
            Ok(read_dir) => read_dir,
            Err(err) => return vec![Err(err)],
        };
        do_recursive_read_and_collect(read_dir)
    } else if path.is_file() {
        vec![dir_entry]
    } else {
        vec![Err(io::Error::new(
            io::ErrorKind::Other,
            "DirEntry was neither a file or a directory.",
        ))]
    }
}
