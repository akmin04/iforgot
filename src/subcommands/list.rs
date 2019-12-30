use super::Command;
use log::info;

pub fn run(commands: &[Command]) {
    info!("Running LIST subcommand");
    if commands.is_empty() {
        println!("No commands are stored");
        return;
    }
    println!("{} commands stored:\n", commands.len());
    let last_id = commands.last().unwrap().id;
    for i in commands {
        println!("{}", i);
        if i.id != last_id {
            println!("");
        }
    }
}
