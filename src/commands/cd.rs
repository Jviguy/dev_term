use dev_term_io::command_io;
use dev_term_io::Executable;
use std::env::set_current_dir;

command_io! {
    struct Cd : "Changes the current working directory", "cd <dir>" {
        pub path: String, "the given path to change the cw to!",
    }
}

impl Executable for Cd {
    fn execute(&self) -> anyhow::Result<()> {
        set_current_dir(&self.path)?;
        Ok(())
    }
}