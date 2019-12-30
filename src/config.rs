use crate::subcommands::Command;
use log::{error, info};
use serde_json::Error;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process;

const FILE_NAME: &str = ".iforgot";
const BLANK_JSON: &str = "[]\n";

fn path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(Path::new(FILE_NAME));
    path
}

fn path_string() -> String {
    path().into_os_string().into_string().unwrap()
}

pub fn read() -> Result<Vec<Command>, Error> {
    info!("Getting file from {}", path_string());
    let mut f = File::open(path()).unwrap_or_else(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            File::create(path()).unwrap_or_else(|_| {
                error!("Could not create config file");
                process::exit(1)
            })
        } else {
            error!("Could not open config file");
            process::exit(1)
        }
    });

    info!("Reading file");
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap_or_else(|_| {
        contents = String::from(BLANK_JSON);
        info!("Creating new config file");
        fs::write(path(), BLANK_JSON).unwrap_or_else(|_| {
            error!("Could not write to config file");
            process::exit(1)
        });
        BLANK_JSON.len()
    });

    info!("Parsing contents");
    let commands: Vec<Command> = serde_json::from_str(&contents)?;

    Ok(commands)
}

pub fn write(commands: &Vec<Command>) {
    info!("Writing commands to config file at {}", path_string());
    let contents = serde_json::to_string(commands).unwrap();
    fs::write(path(), contents).unwrap_or_else(|_| {
        error!("Could not write to config file");
        process::exit(1)
    });
}
