use std::fmt::Debug;
use std::io::Read;
use dev_term_io::{command_io, Executable};
use colored::Colorize;
use rand::Rng;

command_io! {
    struct Dirb : "Brute-force a web server for directories", "dirb <address> [word list]" {
        pub address: String, "The ip address to do the scan on",
        pub word_list: Option<String>, "The url to the word list",
    }
}

impl Executable for Dirb {
    fn execute(&self) -> anyhow::Result<()> {
        let path = self.word_list.clone().unwrap_or("https://raw.githubusercontent.com/danielmiessler/SecLists/master/Discovery/Web-Content/common.txt".to_string());
        let line = "─────────────────────────".red();
        println!("{}\n{}: {}\n{}: {}\n{}\n\n\n\n{}", line, "Address".yellow(), self.address, "Word List".yellow(), &path, line, "[-] ~ ".red());

        let client = reqwest::blocking::Client::new();

        let mut word_string= String::new();
        client.get(&path).send()?.read_to_string(&mut word_string)?;
        let word_list = word_string.split('\n').collect::<Vec<&str>>();

        let mut proxy_string = String::new();
        client.get("https://api.proxyscrape.com/?request=displayproxies&proxytype=socks5&timeout=10000&country=all").send()?.read_to_string(&mut proxy_string)?;
        let proxies = proxy_string.split('\n').collect::<Vec<&str>>();

        let mut rng = rand::thread_rng();
        let total_words = word_list.len();
        for (k, i) in word_list.iter().enumerate() {
            let proxy = reqwest::Proxy::http(format!("socks5://{}", &proxies[rng.gen_range(0..proxies.len())]));
            if proxy.is_err() {
                println!("{}: {}", "Failed creating proxy".red(), proxy.err().unwrap());
                continue;
            }

            let client = reqwest::blocking::Client::builder()
                .proxy(proxy.unwrap())
                .build();
            if client.is_err() {
                println!("{}", "Failed creating client".red());
                continue;
            }

            // todo: dont assume https
            let url = format!("https://{}/{}", self.address, i);
            let resp = client.unwrap().get(&url).send();
            let prefix = format!("{}{}/{}{}", "[".red(), k, total_words, "] ~ ".red());
            if resp.is_ok() && resp.unwrap().status().is_success() {
                println!("\x1b[2A{}\n\x1b[2K\n{}", url.green(), prefix);
            } else {
                println!("\x1b[1A\r{}{}", prefix, url.red());
            }
        }

        Ok(())
    }
}