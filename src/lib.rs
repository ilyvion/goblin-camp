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

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(bare_trait_objects)]
#![deny(ellipsis_inclusive_range_patterns)]
//#![warn(missing_docs)]
#![allow(clippy::new_without_default)]

pub mod coordinate;
pub mod data;
pub mod game;
pub mod ui;
pub mod util;

use clap::ArgMatches;

// TODO: Most of these won't make sense in this version. Consider removing most of them.
#[derive(Clone, Debug)]
pub struct Config {
    boot_test: bool,
    dev_mode: bool,
    no_dumps: bool,
    verbosity: u64,
}

impl Config {
    pub fn new(arg_matches: ArgMatches) -> Result<Config, String> {
        Ok(Config {
            boot_test: arg_matches.is_present("boot_test"),
            dev_mode: arg_matches.is_present("dev_mode"),
            no_dumps: arg_matches.is_present("no_dumps"),
            verbosity: arg_matches.occurrences_of("verbose"),
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
