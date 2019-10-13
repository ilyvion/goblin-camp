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

pub mod data;

pub mod coordinate;
pub use coordinate::*;

pub mod game;

use clap::ArgMatches;

#[derive(Clone, Debug)]
pub struct Config {
    boot_test: bool,
    dev_mode: bool,
    no_dumps: bool,
    verbosity: u64,

    window_width: u32,
    window_height: u32,
}

impl Config {
    pub fn new(arg_matches: ArgMatches) -> Result<Config, String> {
        Ok(Config {
            boot_test: arg_matches.is_present("boot_test"),
            dev_mode: arg_matches.is_present("dev_mode"),
            no_dumps: arg_matches.is_present("no_dumps"),
            verbosity: arg_matches.occurrences_of("verbose"),

            // TODO: Read from a settings file
            window_width: 800,
            window_height: 600,
        })
    }

    pub fn logging_level(&self) -> slog::Level {
        match self.verbosity {
            0 => slog::Level::Info,
            1 => slog::Level::Debug,
            2 | _ => slog::Level::Trace,
        }
    }
}
