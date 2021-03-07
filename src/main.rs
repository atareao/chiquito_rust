extern crate confy;
extern crate dirs;
extern crate log;
extern crate rusqlite;
extern crate serde_derive;
extern crate syslog;

use confy::ConfyError;
use log::{LevelFilter, SetLoggerError};
use rusqlite::Connection;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use syslog::{BasicLogger, Facility, Formatter3164};

#[derive(Default, Debug, Serialize, Deserialize)]
struct Settings {
    version: String,
    url: String,
    secret: String,
    key: String,
}

struct Joke {
    id: i32,
    author: String,
    value: String,
    url: String,
    create_at: i32,
    update_at: i32,
}

impl Joke {
    fn init(connection: &Connection, filename: &str) {}
}

fn main() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "chiquito".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");

    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Debug));

    match dirs::config_dir() {
        Some(path) => {
            let mut configdir = path.to_path_buf();
            println!("{}", &configdir.display());
            configdir.push("chiquito");
            match fs::create_dir_all(&configdir) {
                Ok(resultado) => log::debug!("El directorio ha sido creado o existe {}", "ok"),
                Err(e) => panic!("Adios"),
            }
            configdir.push("config.json");
            println!("{}", &configdir.display());
            let mut settings: Settings = confy::load_path(&configdir).unwrap();
            settings.version = "0.1.0".into();
            confy::store_path(configdir, settings);

            println!("Hello, world!");
        }
        None => panic!("Adios"),
    }
}
