use dioxus::prelude::*;

// Expose a `save_dog` endpoint on our server that takes an "image" parameter
#[server]
pub(crate) async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    // Open the `dogs.txt` file in append-only mode, creating it if doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    // and then write a newline to it with the image url
    file.write_fmt(format_args!("{image}\n"));

    Ok(())
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open DB from persisted file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL,
            );",
        ).unwrap();

        // Return the connection
        conn
    }
}
