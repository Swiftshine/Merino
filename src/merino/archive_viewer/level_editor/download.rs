use std::{fs, io::Cursor, path::Path};

use crate::merino::{
    archive_viewer::level_editor::{
        LevelEditor,
        contexts::{
            download_context::{DownloadContext, DownloadMessage},
            log_context::LogCategory,
        },
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
            let send_progress = |value: f32| {
                let _ = tx.send(DownloadMessage::Progress(value));
            };

            async fn do_download(
                url: &'static str,
                extract_path: std::path::PathBuf,
                send_progress: impl Fn(f32),
            ) -> anyhow::Result<()> {
                fn strip_first_component(path: &str) -> Option<&str> {
                    let mut parts = path.splitn(2, '/');
                    parts.next()?;
                    parts.next()
                }

                // download
                send_progress(10.0);

                let response = reqwest::get(url).await?;
                let response = response.error_for_status()?;

                send_progress(30.0);

                let bytes = response.bytes().await?;

                // extract
                send_progress(50.0);

                tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
                    let reader = Cursor::new(bytes);
                    let mut archive = ZipArchive::new(reader)?;

                    for i in 0..archive.len() {
                        let mut file = archive.by_index(i)?;

                        let original_path = file.name();

                        let Some(stripped) = strip_first_component(original_path) else {
                            continue;
                        };

                        let outpath = Path::new(&extract_path).join(stripped);

                        if file.name().ends_with('/') {
                            fs::create_dir_all(&outpath)?;
                        } else {
                            if let Some(parent) = outpath.parent() {
                                fs::create_dir_all(parent)?;
                            }

                            let mut outfile = std::fs::File::create(&outpath)?;
                            std::io::copy(&mut file, &mut outfile)?;
                        }
                    }

                    Ok(())
                })
                .await??;

                Ok(())
            }

            match do_download(url, extract_path, send_progress).await {
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
                    self.log_context
                        .begin_log(LogCategory::Download, "Starting download".to_string());
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
