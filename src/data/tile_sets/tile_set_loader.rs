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

mod parsers;
pub use parsers::Error;

use crate::data::tile_sets::tile_set::TilesetMetadata;
use crate::data::tile_sets::tile_set_loader::parsers::{
    TileSetParser, TileSetParserV1, TileSetParserV2,
};
use std::path::Path;

pub struct TileSetLoader;

impl TileSetLoader {
    fn get_parser<P: AsRef<Path>>(tile_set_path: P) -> Option<Box<dyn TileSetParser>> {
        let tile_set_path = tile_set_path.as_ref();
        let tile_set_v1_path = tile_set_path.join(TileSetParserV1::FILE_NAME);
        let tile_set_v2_path = tile_set_path.join(TileSetParserV2::FILE_NAME);

        if tile_set_v2_path.exists() {
            Some(Box::new(TileSetParserV2::new(tile_set_v2_path)))
        } else if tile_set_v1_path.exists() {
            Some(Box::new(TileSetParserV1::new(tile_set_v1_path)))
        } else {
            None
        }
    }

    pub fn metadata_by_path<P: AsRef<Path>>(
        tile_set_path: P,
    ) -> Result<Box<dyn TilesetMetadata>, Error> {
        let tile_set_path = tile_set_path.as_ref();
        let tile_set_parser = Self::get_parser(tile_set_path);

        if let Some(mut tile_set_parser) = tile_set_parser {
            tile_set_parser.parse_metadata()
        } else {
            Err(Error::NoParser)
        }
    }
}
