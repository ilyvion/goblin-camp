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

use snafu::{ResultExt, Snafu};
use std::path::{Path, PathBuf};

#[derive(Debug, Snafu)]
pub enum Error {
    PathIoError { source: std::io::Error },
    PathParentError,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait PathProvider {
    fn personal_directory(&self) -> &Path;
    fn executable_file(&self) -> &Path;
    fn executable_directory(&self) -> &Path;
}

pub struct Paths {
    //personal_directory: PathBuf,
    executable_file: PathBuf,
    executable_directory: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self> {
        let executable_file = std::env::current_exe().context(PathIoError)?;

        Ok(Self {
            executable_file: executable_file.clone(),
            executable_directory: executable_file
                .parent()
                .ok_or(Error::PathParentError)?
                .to_path_buf(),
        })
    }
}

impl PathProvider for Paths {
    fn personal_directory(&self) -> &Path {
        unimplemented!()
    }

    fn executable_file(&self) -> &Path {
        &self.executable_file
    }

    fn executable_directory(&self) -> &Path {
        &self.executable_directory
    }
}

// Provide paths for:
//
// Executable, GlobalData, Personal, Mods, Saves,
// Screenshots, Font, Config, ExecutableDir, CoreTilesets, Tilesets
