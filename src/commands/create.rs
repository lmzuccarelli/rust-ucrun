use crate::runtime::crun::*;
use crate::Logging;
use anyhow::ensure;
use anyhow::Result;
use std::ffi::OsStr;
use std::path::Path;

pub fn create(
    log: &Logging,
    args: &liboci_cli::Create,
    raw_args: &[impl AsRef<OsStr>],
) -> Result<()> {
    let bundle_path: &Path = args.bundle.as_path().try_into()?;
    let config_path = bundle_path.join("config.json");

    let spec = oci_spec::runtime::Spec::load(&config_path)?;
    //let original_root_path = spec.root();

    println!("creating container : {:#?}", args.container_id);

    if let Some(process) = spec.process().as_ref() {
        if let Some(capabilities) = process.capabilities().as_ref() {
            fn any_is_cap_sys_admin(caps: &Option<oci_spec::runtime::Capabilities>) -> bool {
                caps.as_ref()
                    .is_some_and(|set| set.contains(&oci_spec::runtime::Capability::SysAdmin))
            }

            ensure!(
                !any_is_cap_sys_admin(capabilities.bounding())
                    && !any_is_cap_sys_admin(capabilities.effective())
                    && !any_is_cap_sys_admin(capabilities.inheritable())
                    && !any_is_cap_sys_admin(capabilities.permitted())
                    && !any_is_cap_sys_admin(capabilities.ambient()),
                "ucrun does not allow privileged containers"
            );
        }
    }

    // save the config
    spec.save(&config_path)?;
    // create the container
    crun(log, raw_args)?;
    Ok(())
}
