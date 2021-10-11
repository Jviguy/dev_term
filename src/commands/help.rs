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
        let mut command: std::io::Result<crate::commands::Command> = Ok(crate::commands::Command::Help(Help::default()));
        match &self.command {
            Some(c) => {
                let mut args = regex.find_iter(c.as_str());
                command = crate::commands::Command::get_cmd(&mut args);
            }
            None => (),
        };
        if let Ok(inner) = command {
            println!("{}", inner.help()?)
        } else if let Err(_) = command {
            println!("That command isn't in dev_term returning windows cmd help!");
            let mut win_cmd = Command::new("cmd");
            win_cmd.arg("/C").args(vec!["help", self.command.clone().unwrap().as_str()]);
            win_cmd.status()?;
        }
        Ok(())
    }
}