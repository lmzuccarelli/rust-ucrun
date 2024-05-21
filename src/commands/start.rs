use crate::api::schema::*;
use crate::common::utils::*;
use crate::runtime::crun::*;
use anyhow::Result;
use custom_logger::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::ffi::OsStr;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct UnikernelConfig {
    #[serde(rename = "kind")]
    kind: String,

    #[serde(rename = "apiVersion")]
    api_version: String,

    #[serde(rename = "spec")]
    spec: Spec,
}

#[derive(Serialize, Deserialize)]
pub struct Spec {
    #[serde(rename = "port")]
    port: String,

    #[serde(rename = "hostPort")]
    host_port: String,

    #[serde(rename = "env")]
    env: Option<Vec<Env>>,

    #[serde(rename = "memory")]
    memory: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Env {
    #[serde(rename = "NAME")]
    name: String,

    #[serde(rename = "VALUE")]
    value: String,
}

pub fn start(
    log: &Logging,
    mut ep: EmbeddedParams,
    args: &liboci_cli::Start,
    raw_args: &[impl AsRef<OsStr>],
) -> Result<()> {
    log.info(&format!("starting container : {:#?}", args.container_id));
    log.debug(&format!("container_manager {:#?}", ep.container_manager));

    ep.container_id = args.container_id.clone();

    crun(log, raw_args)?;

    let config_path = get_config_path(log, ep.clone());
    let config = std::fs::read_to_string(config_path.clone());

    if config.as_ref().ok().is_some() {
        let oci_config: OCIConfig = serde_json::from_str(&config.unwrap())?;
        let envs = oci_config.process.env;
        let service_name_pos = envs.iter().position(|e| e.contains("SERVICE_NAME"));
        let port_pos = envs.iter().position(|e| e.contains("PORT"));

        // set $HOME - this is extremely important for ops nanovm
        env::set_var("HOME", ep.clone().home);

        if service_name_pos.is_some() {
            let service_name = envs[service_name_pos.unwrap()]
                .split("=")
                .collect::<Vec<&str>>()[1];
            let port = envs[port_pos.unwrap()].split("=").collect::<Vec<&str>>()[1];

            // start ops nanovm
            let cmd = "ops".to_string();
            let ops_args: Vec<&str>;
            if port.is_empty() {
                ops_args = vec![
                    "instance",
                    "create",
                    &service_name,
                    "-i",
                    &service_name,
                    "-t",
                    "onprem",
                ];
            } else {
                ops_args = vec![
                    "instance",
                    "create",
                    &service_name,
                    "-i",
                    &service_name,
                    "-p",
                    &port,
                    "-t",
                    "onprem",
                ];
            }

            let ops_nanovm = Command::new(cmd.clone()).args(ops_args).spawn()?;
            //log.debug(&format!("ops instance started {:#?}", ops_nanovm.stdout));

            if ops_nanovm.stderr.is_some() {
                log.error(&format!(
                    "ops instance not started : {:?}",
                    ops_nanovm.stderr
                ));
            } else {
                log.info(&format!("ops instance pid {:#?}", ops_nanovm.id()));
                log.info("container & unikernel logs : ");
            }
        } else {
            log.error("SERVICE_NAME & PORT not found in env unikernel not started");
            return Ok(());
        }
    } else {
        log.error("config.json not found unikernel not started");
        return Ok(());
    }

    Ok(())
}
