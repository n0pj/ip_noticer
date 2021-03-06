use regex::Regex;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};

pub struct BuildDNSTransaction {
    base: String,
    start: String,
    add: String,
    remove: String,
    execute: String,
    zone: String,
    name: String,
    ttl: String,
    r#type: String,
}

impl BuildDNSTransaction {
    pub fn new(zone: &str, name: &str, ttl: &str, r#type: &str) -> Self {
        let base = String::from("gcloud dns record-sets transaction");
        let start = String::new();
        let add = String::new();
        let remove = String::new();
        let execute = String::new();
        let zone = String::from(zone);
        let name = String::from(name);
        let ttl = String::from(ttl);
        let r#type = String::from(r#type);

        Self {
            base,

            start,
            add,
            remove,
            execute,

            zone,
            name,
            ttl,
            r#type,
        }
    }

    pub fn start(mut self, zone: &str) -> Self {
        let zone = format!(r#"--zone="{}""#, zone);
        let start = format!("{} start {}\n", self.base, zone);

        self.start = start;
        self
    }

    pub fn add(
        mut self,
        current_global_ip_address: &str,
        zone: &str,
        name: &str,
        ttl: &str,
        r#type: &str,
    ) -> Self {
        let add = "add";
        let zone = format!(r#"--zone="{}""#, zone);
        let name = format!(r#"--name="{}""#, name);
        let ttl = format!(r#"--ttl="{}""#, ttl);
        let r#type = format!(r#"--type="{}""#, r#type);
        let add = format!(
            "{} {} {} {} {} {} {}\n",
            self.base, add, current_global_ip_address, zone, name, ttl, r#type
        );
        self.add = add;
        self
    }

    pub fn remove(
        mut self,
        before_global_ip_address: &str,
        zone: &str,
        name: &str,
        ttl: &str,
        r#type: &str,
    ) -> Self {
        let remove = "remove";
        let zone = format!(r#"--zone="{}""#, zone);
        let name = format!(r#"--name="{}""#, name);
        let ttl = format!(r#"--ttl="{}""#, ttl);
        let r#type = format!(r#"--type="{}""#, r#type);
        let remove = format!(
            "{} {} {} {} {} {} {}\n",
            self.base, remove, before_global_ip_address, zone, name, ttl, r#type
        );
        self.remove = remove;
        self
    }

    pub fn execute(mut self, zone: &str) -> Self {
        let execute = format!(r#"execute --zone="{}""#, zone);
        let execute = format!("{} {}\n", self.base, execute);
        self.execute = execute;
        self
    }

    pub fn write(&self) {
        let path = "gcloud_dns_transaction_commands.sh";
        let mut writer = OpenOptions::new().write(true).open(&path);

        if let Err(_) = writer {
            writer = File::create(&path)
        }

        if let Ok(mut writer) = writer {
            let start = self.start.clone();
            let remove = self.remove.clone();
            let add = self.add.clone();
            let execute = self.execute.clone();
            let result = format!("{}{}{}{}", &start, &remove, &add, &execute);
            let result = result.as_bytes();
            writer.set_len(0).unwrap();
            writer.write_all(result).unwrap();
        }
    }

    pub fn write_current_global_ip_address(&self, current_global_ip_address: &str) {
        let dotenv_str = match fs::read_to_string(".env") {
            Ok(s) => s,
            Err(_) => {
                println!("{}", "Not found .env");
                println!("{}", "Please copy from .env.example");
                panic!()
            }
        };

        let re_newline = Regex::new(r"\n").unwrap();

        let mut new_dotenv_str = String::new();

        let re = Regex::new(r"(?m)(BEFORE_GLOBAL_IP_ADDRESS)").unwrap();
        let re_n = Regex::new(r"(?m)\n").unwrap();
        let dotenv_str_lines = dotenv_str.lines();

        for line in dotenv_str_lines {
            let mat = re.find(&line);
            if line == "" || line == "\r" || line == "\n" {
                continue;
            }

            match mat {
                Some(_) => {
                    let before_global_ip_address_line =
                        format!("BEFORE_GLOBAL_IP_ADDRESS={}", current_global_ip_address);
                    let mat = re_n.find(&line);
                    new_dotenv_str.push_str(&before_global_ip_address_line);

                    // 改行がなかったら追加
                    if let None = mat {
                        new_dotenv_str.push_str("\n");
                    }
                    continue;
                }
                None => {
                    new_dotenv_str.push_str(&line);
                    let mat = re_n.find(&line);

                    // 改行がなかったら追加
                    if let None = mat {
                        new_dotenv_str.push_str("\n");
                    }
                    continue;
                }
            }
        }

        self.save(&new_dotenv_str, ".env")
    }

    pub fn save(&self, text: &str, path: &str) {
        let mut writer = OpenOptions::new().write(true).open(path);

        if let Err(_) = writer {
            writer = File::create(path);
        }

        if let Ok(mut writer) = writer {
            let bytes = text.as_bytes();
            &writer.set_len(0);
            &writer.write_all(&bytes).unwrap();
        }
    }
}
