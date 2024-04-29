use anyhow::{ensure, Result};
use std::ffi::OsStr;
use std::process::Command;

pub fn crun(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let status = Command::new("crun").args(args).spawn()?.wait()?;
    ensure!(status.success(), "crun failed");

    Ok(())
}
