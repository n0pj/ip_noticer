pub struct DockerCommand {}
use regex::Regex;
use std::process::Command;

impl DockerCommand {
    pub fn build() -> () {
        Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg("ip_noticer:latest")
            .arg(".")
            .status()
            .expect("error");
    }

    pub fn stop() -> () {
        let ps_args = vec!["docker", "ps", "-a", "-q"];
        let docker_ps = Command::new("docker").arg("ps").output().expect("error");
        let docker_ps = String::from_utf8(docker_ps.stdout).expect("error");
        // println!("{}", docker_ps);
        let regex = Regex::new("ip_noticer").unwrap();
        let result = regex.is_match(&docker_ps);

        if result {
            return;
        }

        Command::new("docker")
            .arg("stop")
            .arg("ip_noticer")
            .status()
            .expect("error");
    }

    pub fn rm() -> () {
        let ps_args = vec!["docker", "ps", "-a", "-q"];
        let docker_ps = Command::new("docker").arg("ps").output().expect("error");
        let docker_ps = String::from_utf8(docker_ps.stdout).expect("error");
        // println!("{}", docker_ps);
        let regex = Regex::new("ip_noticer").unwrap();
        let result = regex.is_match(&docker_ps);

        if result {
            return;
        }

        Command::new("docker")
            .arg("rm")
            .arg("ip_noticer")
            .status()
            .expect("error");
    }

    pub fn run() -> () {
        let ps_args = vec!["docker", "ps", "-a", "-q"];
        let docker_ps = Command::new("docker").arg("ps").output().expect("error");
        let docker_ps = String::from_utf8(docker_ps.stdout).expect("error");
        // println!("{}", docker_ps);
        let regex = Regex::new("ip_noticer").unwrap();
        let result = regex.is_match(&docker_ps);

        if result {
            return;
        }

        Command::new("docker")
            .arg("run")
            .arg("--name")
            .arg("ip_noticer")
            .arg("-d")
            .arg("ip_noticer")
            .status()
            .expect("error");
        println!("run ip_noticer")
    }
}
