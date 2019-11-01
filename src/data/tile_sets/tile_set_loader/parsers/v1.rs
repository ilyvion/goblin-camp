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

use crate::data::tile_sets::tile_set::TilesetMetadata;
use crate::data::tile_sets::tile_set_loader::parsers::TileSetParser;
use crate::data::tile_sets::tile_set_loader::Error;
use std::path::PathBuf;

pub struct TileSetParserV1 {
    _path: PathBuf,
}

impl TileSetParserV1 {
    pub const FILE_NAME: &'static str = "tileset.dat";

    pub fn new(path: PathBuf) -> Self {
        Self { _path: path }
    }
}

impl TileSetParser for TileSetParserV1 {
    fn parse_metadata(&mut self) -> Result<Box<dyn TilesetMetadata>, Error> {
        unimplemented!("I have not been able to locate any v1 tile sets, they seem exceedingly rare. Unless support is explicitly requested, I will not implement this.")
    }
}
