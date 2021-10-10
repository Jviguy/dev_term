use std::collections::HashMap;

#[derive(Clone)]
pub struct CommandData {
    pub name: String,
    pub description: String,
    pub usage: String
}

pub trait CommandBody {
    fn execute(&self);

    fn data(&self) -> CommandData;
}