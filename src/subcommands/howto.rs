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

pub fn run(matches: &ArgMatches, commands: &Vec<Command>) {
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
            "\r{}[{}/{}] {} ({}/{}/{}/{})",
            clear::CurrentLine,
            index + 1,
            entries.len(),
            entries[index].0.command,
            "\u{2192}".bold().green(),
            "\u{2191}".bold().blue(),
            "\u{2193}".bold().blue(),
            "\u{2190}".bold().red()
        )
        .unwrap();
        stdout.flush().unwrap();

        match 'key: loop {
            match io::stdin().keys().nth(0).unwrap().unwrap() {
                k @ Key::Right | k @ Key::Up | k @ Key::Down | k @ Key::Left => break 'key k,
                _ => (),
            }
        } {
            Key::Right => {
                write!(stdout, "\r\n").unwrap();
                let split: Vec<&str> = entries[index].0.command.split(" ").collect();
                let mut cmd = process::Command::new(&split[0]);
                for i in 1..split.len() {
                    cmd.arg(&split[i]);
                }
                cmd.spawn().unwrap().wait().unwrap();
                write!(stdout, "\r").unwrap();
                drop(stdout);
                process::exit(0);
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
                write!(stdout, "\r\nAborting!\r\n").unwrap();
                drop(stdout);
                process::exit(0);
            }
            _ => (),
        }
    }
}
