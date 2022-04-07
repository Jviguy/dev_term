use std::fmt::Debug;
use std::io::{Error, ErrorKind, Read};
use dev_term_io::{command_io, Executable};
use colored::Colorize;
use rand::Rng;

command_io! {
    struct Dirb : "Brute-force a web server for directories", "dirb <address> [word list]" {
        pub address: String, "The ip address to do the scan on",
        pub word_list: Option<String>, "The url to the word list to",
    }
}

impl Executable for Dirb {
    fn execute(&self) -> std::io::Result<()> {
        let path = self.word_list.clone().unwrap_or("https://raw.githubusercontent.com/danielmiessler/SecLists/master/Discovery/Web-Content/common.txt".to_string());
        let line = "─────────────────────────".red();
        println!("{}\n{}: {}\n{}: {}\n{}", line, "Address".yellow(), self.address, "Word List".yellow(), &path, line);

        let client = reqwest::blocking::Client::new();

        let word_list;
        let mut word_string = String::new();
        if let Ok(mut response) = client.get(&path).send() {
            if let Err(e) = response.read_to_string(&mut word_string) {
                return Err(Error::new(ErrorKind::Other, e));
            }
            word_list = word_string.split('\n').collect::<Vec<&str>>();
        } else {
            return Err(Error::new(ErrorKind::Other, "Could not get word list"));
        }

        let proxies;
        let mut contents = String::new();
        if let Ok(mut response) = client.get("https://api.proxyscrape.com/?request=displayproxies&proxytype=socks5&timeout=10000&country=all").send() {
            if let Err(e) = response.read_to_string(&mut contents) {
                return Err(Error::new(ErrorKind::Other, e));
            }
            proxies = contents.split('\n').collect::<Vec<&str>>();
        } else {
            return Err(Error::new(ErrorKind::Other, "Could not get proxies"));
        }

        let mut rng = rand::thread_rng();
        for i in word_list {
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
            if resp.is_ok() && resp.unwrap().status().is_success() {
                println!("{}", url.green());
            } else {
                println!("{}", url.red());
            }
        }

        Ok(())
    }
}