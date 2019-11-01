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

pub mod tile_set;
mod tile_set_loader;

use crate::data::paths::{PathProvider, Paths};
use crate::data::tile_sets::tile_set::TilesetMetadata;
use crate::data::tile_sets::tile_set_loader::TileSetLoader;
use slog::{o, warn};
use snafu::{ResultExt, Snafu};
use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{}, Directory: {:?}", source, directory))]
    ReadDirectoryError {
        source: io::Error,
        directory: PathBuf,
    },
    ParsersError {
        source: crate::data::tile_sets::tile_set_loader::Error,
    },
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub struct TileSets {}

impl TileSets {
    pub fn load_tile_set_metadata(
        paths: &Paths,
        parent_logger: slog::Logger,
    ) -> Result<Vec<Box<dyn TilesetMetadata>>> {
        let method_logger = parent_logger.new(o!("Method" => "TileSets::load_tile_set_metadata"));
        let mut tile_set_metadata = vec![];

        // Load core tile sets
        for tile_set_directory in
            fs::read_dir(paths.core_tile_sets_directory()).context(ReadDirectoryError {
                directory: paths.core_tile_sets_directory().to_path_buf(),
            })?
        {
            let tile_set_directory = tile_set_directory.context(ReadDirectoryError {
                directory: paths.core_tile_sets_directory().to_path_buf(),
            })?;

            Self::load_tile_set_metadata_entry(
                &method_logger,
                tile_set_directory,
                &mut tile_set_metadata,
            );
        }

        // Load user tile sets
        for tile_set_directory in
            fs::read_dir(paths.user_tile_sets_directory()).context(ReadDirectoryError {
                directory: paths.user_tile_sets_directory().to_path_buf(),
            })?
        {
            let tile_set_directory = tile_set_directory.context(ReadDirectoryError {
                directory: paths.user_tile_sets_directory().to_path_buf(),
            })?;

            Self::load_tile_set_metadata_entry(
                &method_logger,
                tile_set_directory,
                &mut tile_set_metadata,
            );
        }

        Ok(tile_set_metadata)
    }

    fn load_tile_set_metadata_entry(
        logger: &slog::Logger,
        tile_set_directory: DirEntry,
        tile_set_metadata: &mut Vec<Box<dyn TilesetMetadata>>,
    ) {
        let tile_set_metadata_entry = TileSetLoader::metadata_by_path(tile_set_directory.path());
        match tile_set_metadata_entry {
            Ok(e) => tile_set_metadata.push(e),
            Err(e) => {
                let directory_name = tile_set_directory.file_name();
                let directory_name = directory_name.to_string_lossy();
                match e {
                    tile_set_loader::Error::Parser { source } => warn!(
                        logger,
                        "Could not load metadata for tile set '{}': {}", directory_name, source
                    ),
                    _ => warn!(
                        logger,
                        "Could not load metadata for tile set '{}', error encountered: {}",
                        directory_name,
                        e
                    ),
                }
            }
        }
    }
}
