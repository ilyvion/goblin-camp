/*
    Copyright 2010-2011 Ilkka Halila
    Copyright 2019 Alexander Krivács Schrøder

    This file is part of Goblin Camp.

    Goblin Camp is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Goblin Camp is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Goblin Camp.  If not, see <https://www.gnu.org/licenses/>.
*/

use goblin_camp::game::Game;
use goblin_camp::data::random::DefaultGenerator;
use goblin_camp::data::paths::Paths;
use std::error::Error;
use std::process;
use clap::{App, load_yaml};
use goblin_camp::Config;
use slog::{o, Drain, info};

fn main() -> Result<(), Box<dyn Error>> {
    // Create all "singleton" types
    let _generator = DefaultGenerator::default();
    let _paths = Paths::new()?;

    // Load phase.
    let config = {
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();
        Config::new(matches)
    }.unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Set up logging framework
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build();
    let drain = slog::LevelFilter::new(drain, config.logging_level()).fuse();
    let root_logger = slog::Logger::root(drain, o!());

    info!(root_logger, "Starting {} {}", Game::NAME, Game::VERSION);

    // - Load settings
    // - Load font?
    // - Parse command line? (boottest, dev, nodumps)
    // - Show loading screen while doing heavy I/O?

    // Show main menu, unless boottest, else shut down
    let mut game = Game::new(root_logger.clone(), config);
    game.run().unwrap_or_else(|err| {
        eprintln!("Error while running the game: {}", err);
        process::exit(1);
    });

    info!(root_logger, "Ending {} {}", Game::NAME, Game::VERSION);
    Ok(())
}
