use crate::build_command::BuildCommand;

pub struct BuildDNSTransaction {}

impl BuildDNSTransaction {
    pub fn start() {
        let start = BuildCommand::new();
        println!("{:?}", &start.base)
    }
    pub fn add() {}
    pub fn remove() {}
    pub fn execute() {}
}
