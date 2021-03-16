extern crate dotenv;
mod build_dns_transaction;
mod docker_command;
extern crate clap;
extern crate regex;

use build_dns_transaction::BuildDNSTransaction;
use clap::{App, Arg};
use docker_command::DockerCommand;
use dotenv::dotenv;
use std::env;
use std::process::Command;
// extern crate build_command;

fn main() {
    dotenv().ok();

    let app = App::new("ip_noticer").arg(
        Arg::with_name("command")
            .short("c")
            .takes_value(true)
            .default_value("main"),
    );

    let matches = app.get_matches();
    let command = matches.value_of("command").unwrap();

    println!("{}", command);

    match command {
        "main" => {
            DockerCommand::build();
            DockerCommand::stop();
            DockerCommand::rm();
            DockerCommand::run();
            println!("Start ip_noticer...")
        }
        "generate-only" => {
            let before_global_ip_address = env::var("BEFORE_GLOBAL_IP_ADDRESS").unwrap();
            let current_global_ip_address = Command::new("curl")
                .arg("inet-ip.info")
                .output()
                .expect("error");

            let current_global_ip_address =
                String::from_utf8(current_global_ip_address.stdout).unwrap();
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
            build_dns_transaction = build_dns_transaction.remove(
                &before_global_ip_address,
                &zone,
                &name,
                &ttl,
                &r#type,
            );
            build_dns_transaction =
                build_dns_transaction.add(&current_global_ip_address, &zone, &name, &ttl, &r#type);
            build_dns_transaction = build_dns_transaction.execute(&zone);
            build_dns_transaction.write();
            build_dns_transaction.write_current_global_ip_address(&current_global_ip_address);

            println!("before global ip address: {:?}", before_global_ip_address);
            println!("current global ip address: {:?}", current_global_ip_address);
        }
        &_ => {}
    }
}
