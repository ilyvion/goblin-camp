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

use crate::game::game_data::entity::Entity;
use crate::game::game_data::map::MapGraphicDrawable;
use tcod::{colors, Color};

pub struct NatureObject {
    entity: Entity,

    nature_object_type: i32,
    graphic: char,
    color: Color,
    marked: bool,
    condition: i32,
    tree: bool,
    harvestable: bool,
    ice: bool,
    // static std::vector<NatureObjectPreset> Presets;
}

impl MapGraphicDrawable for NatureObject {
    fn graphic(&self) -> char {
        self.graphic
    }

    fn fore_color(&self) -> Color {
        self.color
    }

    fn back_color(&self) -> Color {
        if self.marked {
            colors::WHITE
        } else {
            colors::BLACK
        }
    }
}
