pub struct BuildCommand {
    pub base: String,
}

impl BuildCommand {
    pub fn new() -> Self {
        let base = String::from("gcloud dns record-sets transaction start --zone=\"nserver\"");
        Self { base }
    }
}

pub mod build_command {
    impl super::BuildCommand {}
}
