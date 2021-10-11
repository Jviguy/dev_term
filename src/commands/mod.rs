use dev_term_io::{command_io, Executable};
mod neofetch;
use neofetch::*;
mod cd;
use cd::*;
mod exit;
use exit::*;
mod help;
use help::*;
mod clear;
use clear::*;
mod ls;
use ls::*;
mod about;
use about::*;
mod ascii;
use ascii::*;

command_io! {
    enum Command : String {
        Neofetch = "neofetch",
        Cd = "cd",
        Exit = "exit",
        Help = "help",
        Clear = "clear",
        Ls = "ls",
        About = "about",
        Ascii = "ascii",
    }
}