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

use serde_derive::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::path::{Path, PathBuf};
use std::{fs, mem};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Cannot save to {:?} because: {}", path, source))]
    SettingsSave {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("Cannot load from {:?} because: {}", path, source))]
    SettingsLoad {
        source: std::io::Error,
        path: PathBuf,
    },
    TomlDeserializationError {
        source: toml::de::Error,
    },
    TomlSerializationError {
        source: toml::ser::Error,
    },
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Renderer {
    GlSl,
    OpenGL,
    SDL,
}

impl Renderer {
    pub fn all() -> impl Iterator<Item = Self> {
        use Renderer::*;
        [GlSl, OpenGL, SDL].iter().cloned()
    }

    pub fn label(self) -> &'static str {
        match self {
            Renderer::GlSl => "GLSL",
            Renderer::OpenGL => "OpenGL",
            Renderer::SDL => "SDL",
        }
    }

    pub fn from_index(i: usize) -> Option<Self> {
        use Renderer::*;
        match i {
            0 => Some(GlSl),
            1 => Some(OpenGL),
            2 => Some(SDL),
            _ => None,
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Renderer::SDL
    }
}

impl From<Renderer> for tcod::Renderer {
    fn from(renderer: Renderer) -> Self {
        match renderer {
            Renderer::GlSl => tcod::Renderer::GLSL,
            Renderer::OpenGL => tcod::Renderer::OpenGL,
            Renderer::SDL => tcod::Renderer::SDL,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub fullscreen: bool,
    pub renderer: Renderer,
    pub use_tile_set: bool,
    pub tile_set: Option<String>,
    pub tutorial: bool,
    pub river_width: u32,
    pub river_depth: u32,
    pub half_rendering: bool,
    pub compress_saves: bool,
    pub translucent_ui: bool,
    pub auto_save: bool,
    pub pause_on_danger: bool,

    pub key_bindings: KeyBindings,
}

impl Settings {
    pub fn load<P: AsRef<Path>>(settings_file_path: P) -> Result<Self> {
        let settings_file_path = settings_file_path.as_ref();
        let settings_string =
            fs::read_to_string(settings_file_path).with_context(|| SettingsLoad {
                path: settings_file_path.to_path_buf(),
            })?;
        let settings: Settings =
            toml::from_str(&settings_string).context(TomlDeserializationError)?;

        Ok(settings)
    }

    pub fn save<P: AsRef<Path>>(&self, settings_file_path: P) -> Result {
        let settings_file_path = settings_file_path.as_ref();
        let settings_string = toml::to_string_pretty(&self).context(TomlSerializationError)?;
        fs::write(settings_file_path, settings_string).with_context(|| SettingsSave {
            path: settings_file_path.to_path_buf(),
        })?;

        Ok(())
    }

    pub fn restore_from(&mut self, other: Settings) {
        mem::replace(self, other);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            resolution_x: 800,
            resolution_y: 600,
            fullscreen: false,
            renderer: Renderer::default(),
            use_tile_set: false,
            tile_set: None,
            tutorial: false,
            river_width: 30,
            river_depth: 5,
            half_rendering: false,
            compress_saves: false,
            translucent_ui: false,
            auto_save: true,
            pause_on_danger: false,

            key_bindings: KeyBindings::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KeyBindings {
    pub exit: char,
    pub basics: char,
    pub workshops: char,
    pub orders: char,
    pub furniture: char,
    pub stock_manager: char,
    pub squads: char,
    pub announcements: char,
    pub center: char,
    pub help: char,
    pub pause: char,
    pub jobs: char,
    pub dev_console: char,
    pub terrain_overlay: char,
    pub permanent: char,
}

impl KeyBindings {
    pub fn key_map<'s>(&'s self) -> impl Iterator<Item = (&'static str, char)> + 's {
        let mut counter = 0;
        std::iter::from_fn(move || {
            let result = match counter {
                0 => Some(("Announcements", self.announcements)),
                1 => Some(("Center", self.center)),
                2 => Some(("Stock Manager", self.stock_manager)),
                3 => Some(("Exit", self.exit)),
                4 => Some(("Permanent", self.permanent)),
                5 => Some(("Furniture", self.furniture)),
                6 => Some(("Orders", self.orders)),
                7 => Some(("Help", self.help)),
                8 => Some(("Basics", self.basics)),
                9 => Some(("Pause", self.pause)),
                10 => Some(("Terrain Overlay", self.terrain_overlay)),
                11 => Some(("Developer Console", self.dev_console)),
                12 => Some(("Jobs", self.jobs)),
                13 => Some(("Squads", self.squads)),
                14 => Some(("Workshops", self.workshops)),
                _ => None,
            };
            if result.is_some() {
                counter += 1;
            }

            result
        })
    }

    pub fn update_key_map(&mut self, mapping: usize, value: char) {
        match mapping {
            0 => self.announcements = value,
            1 => self.center = value,
            2 => self.stock_manager = value,
            3 => self.exit = value,
            4 => self.permanent = value,
            5 => self.furniture = value,
            6 => self.orders = value,
            7 => self.help = value,
            8 => self.basics = value,
            9 => self.pause = value,
            10 => self.terrain_overlay = value,
            11 => self.dev_console = value,
            12 => self.jobs = value,
            13 => self.squads = value,
            14 => self.workshops = value,
            _ => panic!("update_key_map called with out of bounds 'mapping' value"),
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            exit: 'q',
            basics: 'b',
            workshops: 'w',
            orders: 'o',
            furniture: 'f',
            stock_manager: 's',
            squads: 'm',
            announcements: 'a',
            center: 'c',
            help: 'h',
            pause: ' ',
            jobs: 'j',
            dev_console: '`',
            terrain_overlay: 't',
            permanent: 'p',
        }
    }
}
