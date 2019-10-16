/*
    Copyright 2010-2011 Ilkka Halila
    Copyright 2019 Alexander Krivács Schrøder

    This file is part of Goblin Camp Revival.

    Goblin Camp Revival is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Goblin Camp Revival is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Goblin Camp Revival.  If not, see <https://www.gnu.org/licenses/>.
*/

use clap::{load_yaml, App};
use goblin_camp_revival::data::Data;
use goblin_camp_revival::game::Game;
use goblin_camp_revival::Config;
use slog::{info, o, Drain};
use snafu::{ResultExt, Snafu};
use std::process;

fn run() -> Result<(), InitializationError> {
    let config = {
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();
        Config::new(matches).context(ArgumentParsing)?
    };

    // Set up logging framework
    let root_logger = {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build();
        let drain = slog::LevelFilter::new(drain, config.logging_level()).fuse();
        slog::Logger::root(drain, o!())
    };

    info!(root_logger, "Starting {} {}", Game::NAME, Game::VERSION);

    // Create all "singleton" types
    let data = Data::new(root_logger.clone()).context(DataInitialization)?;

    // - Show loading screen while doing heavy I/O?

    // Show main menu, unless boottest, else shut down
    let mut game = Game::new(root_logger.clone(), config, data);
    game.run().context(GameRun)?;

    info!(root_logger, "Ending {} {}", Game::NAME, Game::VERSION);
    Ok(())
}

fn main() {
    run().unwrap_or_else(|err| {
        let cause = format!("{}", err);
        let exit_code: i32 = i32::from(&err);
        let source = match err {
            InitializationError::ArgumentParsing { source } => source,
            InitializationError::DataInitialization { source } => Box::from(source),
            InitializationError::GameRun { source } => Box::from(source),
        };
        eprintln!("Error occurred while {}: {}", cause, source);
        process::exit(exit_code);
    });
}

#[derive(Debug, Snafu)]
pub enum InitializationError {
    #[snafu(display("parsing arguments"))]
    ArgumentParsing {
        #[snafu(source(from(String, Box::from)))]
        source: Box<dyn std::error::Error>,
    },
    #[snafu(display("initializing data"))]
    DataInitialization {
        source: goblin_camp_revival::data::DataError,
    },
    #[snafu(display("running the game"))]
    GameRun {
        source: goblin_camp_revival::game::Error,
    },
}

impl From<&InitializationError> for i32 {
    fn from(initialization_error: &InitializationError) -> Self {
        match initialization_error {
            InitializationError::ArgumentParsing { .. } => 1,
            InitializationError::DataInitialization { .. } => 2,
            InitializationError::GameRun { .. } => 3,
        }
    }
}
