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

use crate::game::Game;
use directories::{BaseDirs, UserDirs};
use snafu::{ResultExt, Snafu};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Snafu)]
pub enum Error {
    PathIoError { source: std::io::Error },
    PathParentError,
    PathDirsError,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait PathProvider {
    fn personal_directory(&self) -> &Path;
    fn executable_file(&self) -> &Path;
    fn executable_directory(&self) -> &Path;
    fn saves_directory(&self) -> &Path;
    fn screenshots_directory(&self) -> &Path;
    fn mods_directory(&self) -> &Path;
    fn user_tile_sets_directory(&self) -> &Path;

    fn settings_file(&self) -> &Path;
    fn font_file(&self) -> &Path;
}

#[derive(Debug)]
pub struct Paths {
    personal_directory: PathBuf,
    executable_file: PathBuf,
    executable_directory: PathBuf,
    saves_directory: PathBuf,
    screenshots_directory: PathBuf,
    mods_directory: PathBuf,
    user_tile_sets_directory: PathBuf,

    settings_file: PathBuf,
    font_file: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self> {
        let executable_file = std::env::current_exe().context(PathIoError)?;
        let executable_directory = executable_file
            .parent()
            .ok_or(Error::PathParentError)?
            .to_path_buf();

        let base_dirs = BaseDirs::new().ok_or(Error::PathDirsError)?;
        let personal_directory = if cfg!(windows) {
            let user_dirs = UserDirs::new().ok_or(Error::PathDirsError)?;

            // TODO: Do this properly? Is there some way to get the "My Games" path in such a way as to be culture independent?
            user_dirs
                .document_dir()
                .ok_or(Error::PathDirsError)?
                .to_path_buf()
                .join("My Games")
        } else {
            base_dirs.home_dir().to_path_buf()
        }
        .join(Game::NAME);

        let saves_directory = personal_directory.join("saves");
        let screenshots_directory = personal_directory.join("screenshots");
        let mods_directory = personal_directory.join("mods");
        let user_tile_sets_directory = personal_directory.join("tilesets");

        fs::create_dir_all(&saves_directory).context(PathIoError)?;
        fs::create_dir_all(&screenshots_directory).context(PathIoError)?;
        fs::create_dir_all(&mods_directory).context(PathIoError)?;
        fs::create_dir_all(&user_tile_sets_directory).context(PathIoError)?;

        let settings_file = personal_directory.join("settings.toml");
        let font_file = personal_directory.join("terminal.png");

        Ok(Self {
            executable_file: executable_file.clone(),
            executable_directory,
            // TODO: ? data_directory,
            // TODO: ? core_tile_sets_directory,
            personal_directory,
            saves_directory,
            screenshots_directory,
            mods_directory,
            user_tile_sets_directory,

            settings_file,
            font_file,
        })
    }
}

impl PathProvider for Paths {
    fn personal_directory(&self) -> &Path {
        &self.personal_directory
    }

    fn executable_file(&self) -> &Path {
        &self.executable_file
    }

    fn executable_directory(&self) -> &Path {
        &self.executable_directory
    }

    fn saves_directory(&self) -> &Path {
        &self.saves_directory
    }

    fn screenshots_directory(&self) -> &Path {
        &self.screenshots_directory
    }

    fn mods_directory(&self) -> &Path {
        &self.mods_directory
    }

    fn user_tile_sets_directory(&self) -> &Path {
        &self.user_tile_sets_directory
    }

    fn settings_file(&self) -> &Path {
        &self.settings_file
    }

    fn font_file(&self) -> &Path {
        &self.font_file
    }
}

// Provide paths for:
//
// Executable, GlobalData, Personal, Mods, Saves,
// Screenshots, Font, Config, ExecutableDir, CoreTilesets, Tilesets
