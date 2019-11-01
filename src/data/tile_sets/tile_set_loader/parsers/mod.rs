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

mod v1;
pub use v1::*;

mod v2;
pub use v2::*;

use crate::data::tile_sets::tile_set::TilesetMetadata;
use snafu::Snafu;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum Error {
    Io {
        source: std::io::Error,
    },
    Parser {
        source: serde_tcod_config_parser::de::Error,
    },
    NoParser,
    PathParentError {
        child: PathBuf,
    },
}

pub trait TileSetParser {
    fn parse_metadata(&mut self) -> Result<Box<dyn TilesetMetadata>, Error>;
}
