use dev_term_io::command_io;
use std::process::Command;
use dev_term_io::Executable;
command_io! {
    struct Help : "Displays information on a given command in the terminal.", "help cmd" {
        pub command: String, "the given command to get information on!",
    }
}

impl Executable for Help {
    fn execute(&self) -> std::io::Result<()> {
        let regex = regex::Regex::new(r#"(?m)("[^"]+"|[^\s"]+)"#).unwrap();
        let mut args = regex.find_iter(self.command.as_str());
        let command = crate::commands::Command::get_cmd(&mut args);
        if let Ok(inner) = command {
                println!("{}", inner.help()?)
        } else if let Err(_e) = command {
            let mut win_cmd = Command::new("cmd");
            win_cmd.arg("/C").args(vec!["help", self.command.as_str()]);
            win_cmd.status()?;
            //check error type as to determine wether we should closest_match
        }
        Ok(())
    }
}