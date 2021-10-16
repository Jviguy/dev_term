use dev_term_io::command_io;
use std::process::Command;
use dev_term_io::Executable;

command_io! {
    struct Help : "Displays information on a given command in the terminal.", "help cmd" {
        pub command: Option<String>, "the given command to get information on!",
    }
}

impl Executable for Help {
    fn execute(&self) -> std::io::Result<()> {
        let regex = regex::Regex::new(r#"(?m)("[^"]+"|[^\s"]+)"#).unwrap();
        match &self.command {
            Some(c) => {
                let mut args = regex.find_iter(c.as_str());
                let command = crate::commands::Command::get_cmd(&mut args);
                if let Ok(inner) = command {
                    println!("{}", inner.help()?)
                } else if let Err(_) = command {
                    println!("That command isn't in dev_term returning windows cmd help!");
                    let mut win_cmd = Command::new("cmd");
                    win_cmd.arg("/C").args(vec!["help", self.command.clone().unwrap().as_str()]);
                    win_cmd.status()?;
                }
            }
            None => {
                // todo: list all commands
                println!("Usage: help <command>");
            },
        };
        Ok(())
    }
}