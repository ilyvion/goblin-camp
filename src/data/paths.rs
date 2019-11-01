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

use crate::game::Game;
use directories::ProjectDirs;
use snafu::{ResultExt, Snafu};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Errors settings up {:?} because: {}", path, source))]
    PathIoError {
        source: std::io::Error,
        path: Option<PathBuf>,
    },
    PathParentError {
        child: PathBuf,
    },
    PathDirsError,
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub trait PathProvider {
    fn executable_file(&self) -> &Path;
    fn executable_directory(&self) -> &Path;
    fn core_tile_sets_directory(&self) -> &Path;

    fn saves_directory(&self) -> &Path;
    fn screenshots_directory(&self) -> &Path;
    fn mods_directory(&self) -> &Path;
    fn user_tile_sets_directory(&self) -> &Path;

    fn settings_file(&self) -> &Path;
    fn font_file(&self) -> &Path;
}

#[derive(Debug)]
pub struct Paths {
    executable_file: PathBuf,
    executable_directory: PathBuf,
    core_tile_sets_directory: PathBuf,

    saves_directory: PathBuf,
    screenshots_directory: PathBuf,
    mods_directory: PathBuf,
    user_tile_sets_directory: PathBuf,

    settings_file: PathBuf,
    font_file: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self> {
        let executable_file = std::env::current_exe().context(PathIoError { path: None })?;
        let executable_directory = executable_file
            .parent()
            .ok_or(Error::PathParentError {
                child: executable_file.clone(),
            })?
            .to_path_buf();

        let core_tile_sets_directory = if cfg!(windows) {
            executable_directory.join("lib").join("tilesets_core")
        } else if cfg!(macos) {
            // TODO: Figure out how this is done on MacOS
            unimplemented!("MacOS support for Paths")
        } else if cfg!(linux) {
            executable_directory
                .parent()
                .ok_or(Error::PathParentError {
                    child: executable_directory.clone(),
                })?
                .join("share")
                .join("goblin-camp-revival")
        } else {
            unimplemented!("support for Paths on your OS")
        };

        let project_dirs = ProjectDirs::from("", "", Game::NAME).ok_or(Error::PathDirsError)?;
        let data_dir = project_dirs.data_dir();
        let config_dir = project_dirs.config_dir();

        let saves_directory = data_dir.join("saves");
        let screenshots_directory = data_dir.join("screenshots");
        let mods_directory = data_dir.join("mods");
        let user_tile_sets_directory = data_dir.join("tilesets");

        fs::create_dir_all(data_dir).with_context(|| PathIoError {
            path: Some(data_dir.to_path_buf()),
        })?;
        fs::create_dir_all(config_dir).with_context(|| PathIoError {
            path: Some(config_dir.to_path_buf()),
        })?;

        fs::create_dir_all(&saves_directory).with_context(|| PathIoError {
            path: Some(saves_directory.clone()),
        })?;
        fs::create_dir_all(&screenshots_directory).with_context(|| PathIoError {
            path: Some(screenshots_directory.clone()),
        })?;
        fs::create_dir_all(&mods_directory).with_context(|| PathIoError {
            path: Some(mods_directory.clone()),
        })?;
        fs::create_dir_all(&user_tile_sets_directory).with_context(|| PathIoError {
            path: Some(user_tile_sets_directory.clone()),
        })?;

        let settings_file = config_dir.join("settings.toml");
        let font_file = data_dir.join("terminal.png");

        Ok(Self {
            executable_file: executable_file.clone(),
            executable_directory,
            core_tile_sets_directory,
            // TODO: ? data_directory,
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
    fn executable_file(&self) -> &Path {
        &self.executable_file
    }

    fn executable_directory(&self) -> &Path {
        &self.executable_directory
    }

    fn core_tile_sets_directory(&self) -> &Path {
        &self.core_tile_sets_directory
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
