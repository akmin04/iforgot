use super::Command;
use clap::ArgMatches;
use log::info;

pub fn run(matches: &ArgMatches, commands: &mut Vec<Command>) {
    info!("Running NEW subcommand");
    let command = matches.value_of("command").unwrap().to_owned();
    let keywords: Vec<String> = matches
        .values_of("keywords")
        .unwrap()
        .map(|k| k.to_owned())
        .collect();
    info!("Command: {}", command);
    info!("Keywords: {}", keywords.join(" "));

    commands.push(Command {
        id: commands.len(),
        command,
        keywords,
    });
}
