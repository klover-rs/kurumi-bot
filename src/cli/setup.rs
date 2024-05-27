use crate::conf::config::Config;
use crate::conf::{
    config::{self, CONFIG},
    utils::CONFIG_FILE,
};
use docker_api::Docker;
use std::process::Command;
pub fn setup_local() {
    let mut config = Config::new();
    if let Ok(c) = CONFIG.lock() {
        config = c.clone()
    }
}

const CONTAINER_NAME: &str = "kurumi-db";
pub fn setup_docker() {
    let mut config = Config::new();
    if let Ok(c) = CONFIG.lock() {
        config = c.clone()
    }

    if let Ok(_docker) = Command::new("docker").arg("--version").output() {
        let password = "password";
        let docker = Command::new("docker")
            .args([
                "run",
                "-d",
                "-p",
                &format!(
                    "{}:{}",
                    config.db.port.unwrap_or(5432),
                    config.db.port.unwrap_or(5432)
                ),
                "--name",
                CONTAINER_NAME,
                "-e",
                format!("POSTGRES_PASSWORD={}", password).as_str(),
                "postgres:14.1-alpine",
            ])
            .output()
            .expect("failed to execute process");
    } else {
        println!("Docker is not installed");
        std::process::exit(0);
    }
}

pub fn container_exits() -> bool {
    true
}

pub fn docker_is_installed() -> bool {
    if let Ok(_docker) = Command::new("docker").arg("--version").output() {
        return true;
    } else {
        return false;
    }
}
