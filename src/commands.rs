pub struct Command {}
use std::process::Command as STDCommand;

impl Command {
    pub fn build() -> () {
        STDCommand::new("docker")
            .arg("build")
            .arg("-t")
            .arg("ip_noticer")
            .arg(".")
            .status()
            .expect("error");
    }

    pub fn stop() -> () {
        STDCommand::new("docker")
            .arg("stop")
            .arg("ip_noticer")
            .status()
            .expect("error");
    }

    pub fn rm() -> () {
        STDCommand::new("docker")
            .arg("rm")
            .arg("ip_noticer")
            .status()
            .expect("error");
    }

    pub fn run() -> () {
        STDCommand::new("docker")
            .arg("run")
            .arg("--name")
            .arg("ip_noticer")
            .arg("-d")
            .arg("ip_noticer")
            .status()
            .expect("error");
    }

    pub fn exec() -> () {
        let command = "gcloud dns record-sets transaction start --zone=\"nserver\"";
        STDCommand::new("docker")
            .arg("exec")
            .arg("-d")
            .arg("ip_noticer")
            .arg(&command)
            .status()
            .expect("error");
    }
}

pub mod command {
    impl super::Command {}
}
