use dev_term_io::command_io;
use std::process::Command;
use dev_term_io::Executable;
use prettytable::{Table, row, cell, format};

command_io! {
    struct Help : "Displays information on a given command in the terminal", "help <cmd>" {
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
                    println!("That command isn't in dev_term, outputting windows help command!");
                    let mut win_cmd = Command::new("cmd");
                    win_cmd.arg("/C").args(vec!["help", self.command.clone().unwrap().as_str()]);
                    win_cmd.status()?;
                }
            }
            None => {
                let mut table: Table = Table::new();
                table.set_format(*format::consts::FORMAT_BOX_CHARS);
                table.add_row(row!["Name", "Description", "Usage"]);
                for c in crate::commands::Command::get_all().iter() {
                    table.add_row(row![c.name(), c.description(), c.usage()]);
                }
                table.printstd();
            },
        };
        Ok(())
    }
}