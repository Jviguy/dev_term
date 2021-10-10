use dev_term_io::command_io;
use dev_term_io::Executable;
use std::process::Command;

command_io! {
    struct Clear: "Clear the contents of the current terminal!", "clear" {}
}

impl Executable for Clear {
    fn execute(&self) -> std::io::Result<()> {
        let mut win_cmd = Command::new("cmd");
        win_cmd.arg("/C").args(vec!["cls"]);
        win_cmd.status()?;
        Ok(())
    }
}