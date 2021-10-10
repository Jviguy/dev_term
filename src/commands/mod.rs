use dev_term_io::command_io;
mod neofetch;
use neofetch::*;

pub trait Executable {
    fn execute(&self) -> std::io::Result<()>;
}

command_io! {
    enum Command : String {
        Neofetch = "neofetch",
    }
}