use crate::api::schema::*;
use crate::common::utils::*;
use crate::runtime::crun::*;
use crate::Logging;
use anyhow::ensure;
use anyhow::Result;
use std::env;
use std::ffi::OsStr;
use std::process::{Command, Stdio};

// does what it says
pub fn delete(
    log: &Logging,
    args: &liboci_cli::Delete,
    raw_args: &[impl AsRef<OsStr>],
) -> Result<()> {
    let unikernel_name = get_unikernel_name(log, &args.container_id);
    log.info(&format!("deleting container: {}", args.container_id,));

    crun(log, raw_args)?;

    if unikernel_name.is_ok() {
        let un = unikernel_name.unwrap().clone();
        log.info(&format!("deleting unikernel: {}", &un));
        kill_process(log, &un)?;
    } else {
        log.error("could not find unikernel name");
    }

    Ok(())
}

// get_unikernel_name returns the unikernel name from the file overlay
fn get_unikernel_name(log: &Logging, container_id: &str) -> Result<String> {
    // auto detect arch
    let mut ep = get_unikernel_config(log);
    if ep.is_err() {
        log.error("unikernel config not found");
        return Err(anyhow::anyhow!("unikernel config not found"));
    } else {
        let mut hld = ep.unwrap();
        hld.container_id = container_id.to_string();
        // set $HOME - this is extremely important for ops nanovm
        env::set_var("HOME", hld.home.clone());
        log.debug(&format!("setting home envar {:#?}", hld.home));
        ep = Ok(hld);
    }

    let config_path = get_config_path(log, ep.unwrap());
    log.debug(&format!("config_path {:#?}", config_path.clone()));
    let config = std::fs::read_to_string(config_path.clone());
    if config.as_ref().ok().is_some() {
        let oci_config: OCIConfig = serde_json::from_str(&config.unwrap())?;
        let envs = oci_config.process.env;
        let service_name_pos = envs.iter().position(|e| e.contains("SERVICE_NAME"));
        if service_name_pos.is_some() {
            let service_name = envs[service_name_pos.unwrap()]
                .split("=")
                .collect::<Vec<&str>>()[1];
            Ok(service_name.to_string())
        } else {
            log.error("SERVICE_NAME not found in env");
            Ok("".to_string())
        }
    } else {
        log.error(&format!("could not find {:#?} config.json", container_id));
        Ok("".to_string())
    }
}

// kill_process kills the unikernel process
fn kill_process(log: &Logging, unikernel_name: &str) -> Result<()> {
    let ps_child = Command::new("ps")
        .arg("-ef")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child_one = Command::new("grep")
        .arg(unikernel_name)
        .stdin(Stdio::from(ps_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child_two = Command::new("grep")
        .arg("-v")
        .arg("grep")
        .stdin(Stdio::from(grep_child_one.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = grep_child_two.wait_with_output().unwrap();
    let result = String::from_utf8_lossy(&output.stdout);
    let process = result.split("\n").collect::<Vec<&str>>();
    for p in process {
        if p.contains(unikernel_name) {
            let ps = p.split_whitespace().collect::<Vec<&str>>();
            let pid = ps[1];
            let kill_child = Command::new("kill").arg("-15").arg(pid).spawn().unwrap();
            let kill_output = kill_child.wait_with_output().unwrap();
            let kill_result = String::from_utf8_lossy(&kill_output.stdout);
            log.info(&format!(
                "terminating unikernel with pid {} {}",
                pid, kill_result
            ));
            ensure!(kill_output.status.success());
        }
    }
    Ok(())
}
