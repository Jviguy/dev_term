use dev_term_io::command_io;
use dev_term_io::Executable;
use colored::Colorize;



command_io! {
    struct About: "Provides information about your dev_term and your pc!", "about" {}
}

impl Executable for About {
    fn execute(&self) -> std::io::Result<()> {
        println!("{}", crate::ASCII.green());
        println!("You are running {} of {}", 
            format!("v{}", crate::VERSION).red(),
            "Dev Term!".green());
        Ok(())
    }
}