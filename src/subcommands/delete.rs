use super::Command;
use clap::ArgMatches;
use log::{error, info};
use std::{io, process};

pub fn run(matches: &ArgMatches, commands: &mut Vec<Command>) {
    info!("Running DELETE subcommand");
    if matches.is_present("all") {
        println!("Are you sure you want to delete all entries? (y/n)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_lowercase();
        if input != "y" {
            println!("Aborting");
            process::exit(0);
        }
        info!("Clearing all commands");
        commands.clear();
    } else {
        let id = matches
            .value_of("id")
            .unwrap()
            .parse::<usize>()
            .unwrap_or_else(|_| {
                error!("Not a valid ID");
                process::exit(1)
            });

        info!("Deleting ID: {}", id);
        commands.remove(id);
        for i in id..commands.len() {
            commands[i].id -= 1;
        }
    }
}
