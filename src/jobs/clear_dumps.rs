use log::{error, info};
use std::fs;

pub async fn clear_dumps() {
    let dir = match fs::read_dir("exports/dumps") {
        Ok(d) => d,
        Err(err) => {
            error!(
                "(cron - clear_dumps) Could not read dumps directory: {:?}",
                err
            );
            return;
        }
    };

    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                error!(
                    "(cron - clear_dumps) Could not read directory entry: {:?}",
                    err
                );
                continue;
            }
        };

        let path = entry.path();

        if path.is_file() {
            if entry.file_name() == ".gitkeep" {
                continue;
            }
            match fs::remove_file(&path) {
                Ok(_) => info!(
                    "(cron - clear_dumps) Deleted dump file: {:?}",
                    entry.file_name()
                ),
                Err(err) => error!(
                    "(cron - clear_dumps) Failed to delete {:?}: {:?}",
                    entry.file_name(),
                    err
                ),
            }
        }
    }
}
