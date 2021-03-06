extern crate dotenv;
mod build_dns_transaction;
mod docker_command;
extern crate regex;

use build_dns_transaction::BuildDNSTransaction;
use docker_command::DockerCommand;
use dotenv::dotenv;
use std::env;
use std::process::Command;
// extern crate build_command;

fn main() {
    dotenv().ok();

    let before_global_ip_address = env::var("BEFORE_GLOBAL_IP_ADDRESS").unwrap();
    let current_global_ip_address = Command::new("curl")
        .arg("inet-ip.info")
        .output()
        .expect("error");

    let current_global_ip_address = String::from_utf8(current_global_ip_address.stdout).unwrap();
    let current_global_ip_address = current_global_ip_address.replace("\n", "");
    let zone = env::var("ZONE").unwrap();
    let name = env::var("NAME").unwrap();
    let ttl = env::var("TTL").unwrap();
    let r#type = env::var("TYPE").unwrap();

    let zone = String::from(zone);
    let name = String::from(name);
    let ttl = String::from(ttl);
    let r#type = String::from(r#type);

    let mut build_dns_transaction = BuildDNSTransaction::new(&zone, &name, &ttl, &r#type);
    build_dns_transaction = build_dns_transaction.start(&zone);
    build_dns_transaction =
        build_dns_transaction.remove(&before_global_ip_address, &zone, &name, &ttl, &r#type);
    build_dns_transaction =
        build_dns_transaction.add(&current_global_ip_address, &zone, &name, &ttl, &r#type);
    build_dns_transaction = build_dns_transaction.execute(&zone);
    build_dns_transaction.write();
    build_dns_transaction.write_current_global_ip_address(&current_global_ip_address);

    DockerCommand::build();
    DockerCommand::stop();
    DockerCommand::rm();
    DockerCommand::run();

    println!("before global ip address: {:?}", before_global_ip_address);
    println!("current global ip address: {:?}", current_global_ip_address);
}
