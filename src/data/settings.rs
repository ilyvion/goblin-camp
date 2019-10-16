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

use serde_derive::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::fs;
use std::path::Path;

#[derive(Debug, Snafu)]
pub enum Error {
    SettingsIoError { source: std::io::Error },
    TomlDeserializationError { source: toml::de::Error },
    TomlSerializationError { source: toml::ser::Error },
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Deserialize, Serialize, Debug)]
pub enum Renderer {
    GLSL,
    OpenGL,
    SDL,
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer::SDL
    }
}

impl From<Renderer> for tcod::Renderer {
    fn from(renderer: Renderer) -> Self {
        match renderer {
            Renderer::GLSL => tcod::Renderer::GLSL,
            Renderer::OpenGL => tcod::Renderer::OpenGL,
            Renderer::SDL => tcod::Renderer::SDL,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub fullscreen: bool,
    pub renderer: Renderer,
    pub use_tileset: bool,
    pub tileset: Option<String>,
    pub tutorial: bool,
    pub river_width: u32,
    pub river_depth: u32,
    pub half_rendering: bool,
    pub compress_saves: bool,
    pub translucent_ui: bool,
    pub auto_save: bool,
    pub pause_on_danger: bool,
}

impl Settings {
    pub fn load<P: AsRef<Path>>(settings_file_path: P) -> Result<Self> {
        let settings_string = fs::read_to_string(settings_file_path).context(SettingsIoError)?;
        let settings: Settings =
            toml::from_str(&settings_string).context(TomlDeserializationError)?;

        Ok(settings)
    }

    pub fn save<P: AsRef<Path>>(&self, settings_file_path: P) -> Result {
        let settings_string = toml::to_string_pretty(&self).context(TomlSerializationError)?;
        fs::write(settings_file_path, settings_string).context(SettingsIoError)?;

        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resolution_x: 800,
            resolution_y: 600,
            fullscreen: false,
            renderer: Renderer::default(),
            use_tileset: false,
            tileset: None,
            tutorial: false,
            river_width: 30,
            river_depth: 5,
            half_rendering: false,
            compress_saves: false,
            translucent_ui: false,
            auto_save: true,
            pause_on_danger: false,
        }
    }
}
