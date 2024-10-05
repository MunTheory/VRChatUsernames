use std::io::{BufRead as _, Write as _};

fn main() {
    let directory = std::env::current_exe().expect("unable to get current exe's path");
    let parent = directory
        .parent()
        .expect("unable to get current exe's parent path");

    let mut writer = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(directory.with_extension("db"))
        .map(std::io::BufWriter::new)
        .expect("unable to open file");

    for entry in std::fs::read_dir(parent).expect("unable to read directory") {
        let entry = entry.expect("unable to read an item from directory");

        let name = entry.file_name();
        if !name.to_string_lossy().starts_with("output_log_") {
            continue;
        }

        let reader = std::fs::OpenOptions::new()
            .read(true)
            .open(entry.path())
            .map(std::io::BufReader::new)
            .expect("unable to open file");

        for line in reader.lines() {
            let line = line.expect("unable to read line");

            if let Some(name) = line
                .strip_prefix("[Behaviour] OnPlayerJoined ")
                .or_else(|| line.strip_prefix("[Behaviour] OnPlayerLeft "))
            {
                writer
                    .write_all(name.as_bytes())
                    .and_then(|_| writer.write_all(b"\n"))
                    .expect("unable to write data");
            }
        }
    }
}
