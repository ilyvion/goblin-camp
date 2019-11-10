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

pub mod base;
#[allow(clippy::module_inception)]
pub mod data;
pub mod paths;
pub mod random;
pub mod settings;
pub mod tile_sets;

use paths::{PathProvider, Paths};
use random::DefaultGenerator;
use settings::Settings;

use rand::rngs::StdRng;
use slog::{debug, o};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum DataError {
    PathInitialization { source: paths::Error },
    SettingsLoad { source: settings::Error },
}

pub type Result<T = (), E = DataError> = std::result::Result<T, E>;

pub struct Data {
    pub generator: DefaultGenerator<StdRng>,
    pub paths: Paths,
    pub settings: Settings,
}

impl Data {
    pub fn new(parent_logger: &slog::Logger) -> Result<Self> {
        let logger = parent_logger.new(o!());
        let method_logger = logger.new(o!("Method" => "Data::new"));
        let generator = DefaultGenerator::default();
        let paths = Paths::new().context(PathInitialization)?;
        debug!(method_logger, "{:?}", paths);
        let settings = if paths.settings_file().exists() {
            Settings::load(paths.settings_file()).context(SettingsLoad)?
        } else {
            Settings::default()
        };
        debug!(method_logger, "{:?}", settings);

        Ok(Self {
            generator,
            paths,
            settings,
        })
    }
}
