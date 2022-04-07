use dev_term_io::command_io;
use dev_term_io::Executable;
use walkdir::WalkDir;
use std::path::PathBuf;
use anyhow::anyhow;

command_io! {
    struct Ls: "Displays the current files in a directory", "ls" {
        pub path: Option<String>, "the path to inspect!",
        pub flag: Option<String>, "a flag representing what the next param will be!",
        pub depth: Option<u32>, "the depth of recursion that we should inspect!",
    }
}

impl Executable for Ls {
    fn execute(&self) -> anyhow::Result<()> {
        let path = match &self.path {
            Some(p) => PathBuf::from(p),
            None => std::env::current_dir()?
        };
        let depth = match &self.flag {
            Some(d) => {
                match &**d {
                    "-d" | "--depth" => {
                        match self.depth {
                            Some(de) => de as usize,
                            None => {
                                return Err(anyhow!("Expected non negative integer depth!"));
                            }
                        }
                    }
                    _ => {
                        return Err(anyhow!(format!("Expected a valid flag found: {}!", d)));
                    }
                }
            },
            None => 1,
        };
        for entry in WalkDir::new(&path).max_depth(depth) {
            println!("{}", entry?.path().display());
        }
        Ok(())
    }
}