mod build_command;
mod build_dns_transaction;
mod commands;

use build_dns_transaction::BuildDNSTransaction;
use commands::Command;
// extern crate build_command;

fn main() {
    BuildDNSTransaction::start();
    BuildDNSTransaction::remove();
    BuildDNSTransaction::add();
    BuildDNSTransaction::execute();

    // Command::build();
    // Command::stop();
    // Command::rm();
    // Command::run();
    Command::exec();
    println!("test");
}
