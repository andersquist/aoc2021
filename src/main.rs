use aoc2021::config::Config;
use color_eyre::eyre::{bail, Result};
use path_absolutize::Absolutize;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum ConfigOpt {
    /// Print path to configuration file
    Path,
    /// Display config
    Show,
    /// Set configuration
    Set {
        /// session id - log in to adventofcode.com and inspect cookies to get this
        #[structopt(short, long)]
        session: Option<String>,

        /// Path to input files
        #[structopt(short, long, parse(from_os_str))]
        inputs: Option<PathBuf>,
    },
}

impl ConfigOpt {
    fn run(self) -> Result<()> {
        match self {
            Self::Path => println!("{}", aoc2021::config::path().display()),
            Self::Show => {
                let content = std::fs::read_to_string(aoc2021::config::path())?;
                println!("{}", content);
            }
            Self::Set { session, inputs } => {
                let mut config = Config::load().unwrap_or_default();
                if let Some(session) = session {
                    if session.is_empty() {
                        bail!("session can't be empty")
                    }
                    config.session = session;
                }
                if let Some(inputs) = inputs {
                    if inputs.exists() && !inputs.is_dir() {
                        bail!("inputs must be a directory")
                    }
                    config.input_files = Some(inputs.absolutize()?.into_owned());
                }
                config.save()?;
            }
        }
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
#[structopt(about = "advent of code 2021")]
enum Command {
    /// Handle configuration
    Config {
        #[structopt(subcommand)]
        cmd: ConfigOpt,
    },
    /// Emit the URL to a specified puzzle
    Url {
        #[structopt(long, default_value = "1")]
        day: u8,
    },
    Init {
        #[structopt(long, default_value = "1")]
        day: u8,
        /// Force overwrite files
        #[structopt(long, short)]
        force: bool,
    },
}

impl Command {
    fn run(self) -> Result<()> {
        match self {
            Self::Config { cmd } => cmd.run()?,
            Self::Url { day } => {
                println!("{}", aoc2021::utils::url_for_day(day));
            }
            Self::Init { day, force } => {
                let config = Config::load()?;
                aoc2021::day::initialize(&config, day, force)?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let command = Command::from_args();
    command.run()
}
