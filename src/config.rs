use crate::subcommands::Command;
use log::info;
use std::fs::{self, File};
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};

const FILE_NAME: &str = ".iforgot";
const BLANK_JSON: &str = "[]\n";

macro_rules! unwrap_or_err {
    ( $e:expr, $msg: tt) => {
        match $e {
            Ok(i) => i,
            Err(_) => return Err($msg),
        }
    };
}

fn path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(Path::new(FILE_NAME));
    path
}

fn path_string() -> String {
    path().into_os_string().into_string().unwrap()
}

pub fn read<'a>() -> Result<Vec<Command>, &'a str> {
    info!("Opening file from {}", path_string());
    let mut f = match File::open(path()) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                unwrap_or_err!(File::create(path()), "Could not create config file")
            }
            _ => return Err("Could not open config file"),
        },
    };

    info!("Reading file");
    let mut contents = String::new();
    if f.read_to_string(&mut contents).is_err() {
        contents = String::from(BLANK_JSON);
        info!("Creating new config file");
        unwrap_or_err!(
            fs::write(path(), BLANK_JSON),
            "Could not write to config file"
        );
    }

    info!("Parsing contents");
    let commands: Vec<Command> =
        unwrap_or_err!(serde_json::from_str(&contents), "Could not parse JSON");

    Ok(commands)
}

pub fn write<'a>(commands: &[Command]) -> Result<(), &'a str> {
    info!("Writing commands to config file at {}", path_string());
    let contents = serde_json::to_string(commands).unwrap();

    match fs::write(path(), contents) {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not write to config file"),
    }
}
