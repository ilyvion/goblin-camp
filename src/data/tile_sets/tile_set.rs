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

use crate::data::base::Size;
use std::borrow::Cow;
use std::path::Path;

pub trait TilesetMetadata {
    fn dir(&self) -> &Path;
    fn name(&self) -> &str;
    fn author(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn size(&self) -> Size;

    fn dir_name(&self) -> Cow<str> {
        self.dir().file_name().unwrap().to_string_lossy()
    }
}
