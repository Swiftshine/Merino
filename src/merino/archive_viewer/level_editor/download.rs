use std::{fs, io::Cursor, path::Path};

use crate::merino::{
    archive_viewer::level_editor::{
        LevelEditor,
        contexts::{download_context::{DownloadContext, DownloadMessage}, log_context::LogCategory},
    },
    util::res_folder::get_merino_folder,
};
use anyhow::Result;
use zip::ZipArchive;

pub const OBJECTDB_URL: &str =
    "https://github.com/Swiftshine/yww-objectdb/archive/refs/heads/main.zip";
pub const IMAGEDB_URL: &str =
    "https://github.com/Swiftshine/yww-merino-image/archive/refs/heads/main.zip";

impl LevelEditor {
    pub fn start_download(&mut self, url: &'static str) -> Result<()> {
        // prevent multiple downloads
        if self.download_context.is_some() {
            return Ok(());
        }

        // create communication channel
        let (tx, rx) = std::sync::mpsc::channel();

        // create context
        self.download_context = Some(DownloadContext::new(rx));

        let extract_path = get_merino_folder()?;

        self.runtime.spawn(async move {
            // helper
            let send_progress = |value: f32| {
                let _ = tx.send(DownloadMessage::Progress(value));
            };

            fn strip_first_component(path: &str) -> Option<&str> {
                let mut parts = path.splitn(2, '/');
                parts.next()?; // drop repo root folder
                parts.next()
            }

            // download
            send_progress(10.0);
            let response = reqwest::get(url).await.unwrap();

            send_progress(30.0);
            let bytes = response.bytes().await.unwrap();

            // extract
            send_progress(50.0);

            let result = tokio::task::spawn_blocking(move || {
                let reader = Cursor::new(bytes);
                let mut archive = ZipArchive::new(reader).unwrap();

                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();

                    let original_path = file.name();

                    // skip root
                    let Some(stripped) = strip_first_component(original_path) else {
                        continue;
                    };

                    let outpath = Path::new(&extract_path).join(stripped);

                    if file.name().ends_with('/') {
                        fs::create_dir_all(&outpath).unwrap();
                    } else {
                        if let Some(parent) = outpath.parent() {
                            fs::create_dir_all(parent).unwrap();
                        }

                        let mut outfile = std::fs::File::create(&outpath).unwrap();
                        std::io::copy(&mut file, &mut outfile).unwrap();
                    }
                }
            })
            .await;

            // done

            match result {
                Ok(_) => {
                    send_progress(100.0);
                    let _ = tx.send(DownloadMessage::Finished);
                }

                Err(e) => {
                    let _ = tx.send(DownloadMessage::Error(e.to_string()));
                }
            }
        });
        Ok(())
    }

    pub fn handle_download_messages(&mut self) {
        let mut should_clear = false;
        if let Some(context) = &mut self.download_context {
            while let Ok(msg) = context.receiver().try_recv() {
                if !self.log_context.log_started() {
                    self.log_context.begin_log(LogCategory::Download, "Starting download".to_string());
                }

                match msg {
                    DownloadMessage::Progress(value) => {
                        self.log_context.update_log(format!("Download: {value}%"));
                    }

                    DownloadMessage::Finished => {
                        self.log_context.finish_log("Finished.".to_string());
                        should_clear = true;
                    }

                    DownloadMessage::Error(e) => {
                        self.log_context.finish_log(format!("Download failed: {e}"));
                        should_clear = true;
                    }
                }
            }
        }

        if should_clear {
            self.download_context = None;
        }
    }
}
