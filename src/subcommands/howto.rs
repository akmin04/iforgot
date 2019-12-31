use super::Command;
use clap::ArgMatches;
use colored::*;
use log::info;
use std::io::{self, Write};
use std::process;
use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Entry<'a>(&'a Command, usize);

pub fn run(matches: &ArgMatches, commands: &[Command]) {
    let keywords: Vec<String> = matches
        .values_of("keywords")
        .unwrap()
        .map(|k| k.to_owned())
        .collect();
    info!(
        "Searching for command with these keywords: {}",
        keywords.join(" ")
    );

    let mut entries: Vec<Entry> = Vec::new();
    for c in commands {
        let mut count = 0;
        for k in &c.keywords {
            count += if keywords.contains(k) { 1 } else { 0 };
        }
        if count != 0 {
            info!("Command ID {} has {} common keywords", c.id, count);
            entries.push(Entry(c, count));
        }
    }

    if entries.is_empty() {
        println!("{}", "No commands found!".bold().red());
        process::exit(0);
    }

    info!("Sorting entries");
    entries.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1).reverse()
        } else {
            a.0.id.cmp(&b.0.id)
        }
    });

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut index = 0;
    loop {
        write!(
            stdout,
            "\r{}[{}/{}] {} ({} {} {} {})",
            clear::CurrentLine,
            index + 1,
            entries.len(),
            entries[index].0.command,
            "\u{2190}".bold().red(),
            "\u{2191}".bold().blue(),
            "\u{2193}".bold().blue(),
            "\u{2192}".bold().green(),
        )
        .unwrap();
        stdout.flush().unwrap();

        match io::stdin().keys().nth(0).unwrap().unwrap() {
            Key::Right => {
                writeln!(stdout, "\r").unwrap();
                let split: Vec<&str> = entries[index].0.command.split(' ').collect();
                let mut cmd = process::Command::new(split[0]);
                for item in split.iter().skip(1) {
                    cmd.arg(item);
                }
                drop(stdout);
                info!("Exiting raw mode");
                info!("Spawning process");
                cmd.spawn().unwrap().wait().unwrap();
                print!("\r");
                break;
            }
            Key::Up => {
                index = if index == 0 {
                    entries.len() - 1
                } else {
                    index - 1
                }
            }
            Key::Down => {
                index = if index == entries.len() - 1 {
                    0
                } else {
                    index + 1
                }
            }
            Key::Left => {
                write!(stdout, "\r\n{}\r\n", "Aborting!".bold().red()).unwrap();
                break;
            }
            _ => (),
        }
    }
}
