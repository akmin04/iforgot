use crate::subcommands::*;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use log::LevelFilter::{Error, Trace};

pub fn init() -> ArgMatches<'static> {
    App::new("iforgot")
        .version("0.1.1")
        .about("Recall obscure commands by keyword")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable verbose logging"),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a specified entry by ID")
                .arg(
                    Arg::with_name("all")
                        .long("all")
                        .help("Delete all commands"),
                )
                .arg(
                    Arg::with_name("id")
                        .required_unless("all")
                        .help("ID of command to delete"),
                ),
        )
        .subcommand(
            SubCommand::with_name("howto")
                .about("Get a list of commands with matching keywords")
                .arg(
                    Arg::with_name("keywords")
                        .required(true)
                        .min_values(1)
                        .help("Keywords to search for"),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List all saved entries"))
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new entry")
                .arg(
                    Arg::with_name("command")
                        .short("c")
                        .value_name("command")
                        .help("The command entry")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("keywords")
                        .short("k")
                        .value_name("keywords")
                        .help("List of keywords associated with the command")
                        .required(true)
                        .min_values(1),
                ),
        )
        .get_matches()
}

pub fn init_logger(matches: &ArgMatches) {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .filter_level(if matches.is_present("verbose") {
            println!("Printing verbose outputs");
            Trace
        } else {
            Error
        })
        .init();
}

pub fn handle_subcommands(matches: &ArgMatches, commands: &mut Vec<Command>) {
    match matches.subcommand() {
        ("delete", Some(m)) => delete::run(m, commands),
        ("howto", Some(m)) => howto::run(m, commands),
        ("list", _) => list::run(commands),
        ("new", Some(m)) => new::run(m, commands),
        _ => (),
    }
}
