use crate::Logging;
use anyhow::{ensure, Result};
use std::ffi::OsStr;
use std::process::Command;

pub fn crun(log: &Logging, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<()> {
    let local_args = args
        .into_iter()
        .map(|a| a.as_ref().to_os_string())
        .collect::<Vec<_>>();
    log.info(&format!(
        "executing crun with args : {:#?}",
        local_args.clone()
    ));
    let status = Command::new("crun").args(local_args).spawn()?.wait()?;
    ensure!(status.success(), "crun failed");
    Ok(())
}
