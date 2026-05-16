use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};

const MERINO_FOLDER: &str = "merino_res";

fn merino_folder_path() -> Result<PathBuf> {
    let base_path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        // dev
        PathBuf::from(manifest_dir)
    } else {
        // release
        env::current_exe()?
            .parent()
            .context("Could not find executable parent directory")?
            .to_path_buf()
    };

    Ok(base_path.join(MERINO_FOLDER))
}

fn merino_folder_exists() -> Result<bool> {
    Ok(merino_folder_path()?.exists())
}

fn make_merino_folder() -> Result<()> {
    fs::create_dir(merino_folder_path()?)?;
    Ok(())
}

// Creates `merino_res` if it doesn't exist, and returns a path to it.
pub fn get_merino_folder() -> Result<PathBuf> {
    if !merino_folder_exists()? {
        make_merino_folder()?;
    }

    merino_folder_path()
}
