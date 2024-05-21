use crate::api::schema::EmbeddedParams;
use custom_logger::Logging;

pub fn get_config_path(log: &Logging, ep: EmbeddedParams) -> String {
    let config_path = match ep.container_manager.as_str() {
        "podman" => {
            ep.home.to_string()
                + &".local/share/containers/storage/overlay-containers/".to_string()
                + &ep.container_id
                + &"/userdata/config.json"
        }
        "cri-o" => {
            "/var/run/containers/storage/overlay-containers/".to_string()
                + &ep.container_id
                + &"/userdata/config.json"
        }
        _ => {
            log.error(&format!(
                "container-manager {:#?} not supported unikernel not started",
                ep.container_manager.clone()
            ));
            "".to_string()
        }
    };
    config_path
}

pub fn get_unikernel_config(log: &Logging) -> Result<EmbeddedParams, ()> {
    // retrieve the index.json file
    // used to extract info such as $HOME $USER
    // container caller (i.e podman, cri-o, containerd)
    let index = std::fs::read_to_string("/opt/unikernel/index.json");
    if index.as_ref().ok().is_some() {
        let index_json: serde_json::Value =
            serde_json::from_str(&index.unwrap()).expect("should read index.json");
        let ep = EmbeddedParams {
            container_manager: index_json["containerManager"].to_string().replace("\"", ""),
            home: index_json["home"].to_string().replace("\"", ""),
            container_id: "".to_string(),
            log_level: index_json["logLevel"].to_string().replace("\"", ""),
        };
        log.debug(&format!("unikernel config {:#?}", ep));
        Ok(ep)
    } else {
        log.error(&format!(
            "/opt/unikernel/index.json not found unikernel not started"
        ));
        Err(())
    }
}
