use crate::runtime::crun::*;
use anyhow::ensure;
use anyhow::Result;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn delete(args: &liboci_cli::Delete, raw_args: &[impl AsRef<OsStr>]) -> Result<()> {
    // the container might not exist because creation failed midway through, so we ignore errors
    let root_path = get_root_path(&args.container_id).ok();

    crun(raw_args)?;

    if let Some(root_path) = root_path {
        let private_dir_path: PathBuf = root_path.parent().unwrap().to_path_buf().try_into()?;

        let image_dir_path = private_dir_path.join("root/ucrun/image");
        let image_file_path = image_dir_path.join("image");
        fs::remove_dir(image_file_path)?;
        fs::remove_dir(image_dir_path)?;
    }

    Ok(())
}

fn get_root_path(container_id: &str) -> Result<PathBuf> {
    let output = Command::new("crun")
        .arg("state")
        .arg(container_id)
        .stderr(Stdio::null())
        .output()?;

    ensure!(output.status.success());

    #[derive(Deserialize)]
    struct ContainerState {
        rootfs: PathBuf,
    }

    let state: ContainerState = serde_json::from_slice(&output.stdout)?;

    Ok(state.rootfs.try_into()?)
}
