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
#![deny(unsafe_code)]
//#![warn(missing_docs)]

// Clippy conventions
#![deny(clippy::cast_lossless)]
#![deny(clippy::default_trait_access)]
#![deny(clippy::empty_enum)]
#![deny(clippy::enum_glob_use)]
#![deny(clippy::expl_impl_clone_on_copy)]
#![deny(clippy::explicit_into_iter_loop)]
#![deny(clippy::explicit_iter_loop)]
#![deny(clippy::filter_map)]
#![deny(clippy::filter_map_next)]
#![deny(clippy::find_map)]
#![deny(clippy::if_not_else)]
#![deny(clippy::invalid_upcast_comparisons)]
#![deny(clippy::items_after_statements)]
#![deny(clippy::large_digit_groups)]
#![deny(clippy::map_flatten)]
#![deny(clippy::match_same_arms)]
#![deny(clippy::mut_mut)]
#![deny(clippy::needless_continue)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::option_map_unwrap_or)]
#![deny(clippy::option_map_unwrap_or_else)]
#![deny(clippy::redundant_closure_for_method_calls)]
#![deny(clippy::result_map_unwrap_or_else)]
#![deny(clippy::single_match_else)]
#![deny(clippy::string_add_assign)]
#![deny(clippy::type_repetition_in_bounds)]
#![deny(clippy::unseparated_literal_suffix)]
#![deny(clippy::unused_self)]
//#![deny(clippy::use_self)] // Too many false positives, currently
#![deny(clippy::used_underscore_binding)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::pub_enum_variant_names)]
#![warn(clippy::replace_consts)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::similar_names)]
#![warn(clippy::too_many_lines)]
#![allow(clippy::new_without_default)]
#![allow(clippy::cast_sign_loss)]

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
    pub fn new(arg_matches: &ArgMatches) -> Result<Self, String> {
        Ok(Self {
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
